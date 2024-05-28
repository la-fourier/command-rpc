use quote::{format_ident, quote, ToTokens};
use syn::{
    token::{Brace, Bracket, Comma, Paren},
    Attribute, ExprLit, Field, FieldMutability, Fields, FnArg, Generics, Ident, ImplItem,
    ImplItemFn, Item, Lit, LitStr, Meta, MetaList, MetaNameValue, Pat, PatIdent, PatType, Path,
    PathArguments, PathSegment, Receiver, Token, TypePath, Visibility,
    __private::Span,
    punctuated::Punctuated,
    AttrStyle, Expr, FieldsNamed, ItemFn, ItemStruct, MacroDelimiter, Type,
};

fn type_hint(mut type_str: String) -> String {
    let reference = match type_str.clone().chars().next().unwrap() {
        '&' => "ref | ",
        _ => "",
    };
    type_str = type_str.replace("&", "").replace(" ", "");
    let hint: String = match type_str.as_str() {
        "String" => "string".to_string(),
        "str" => "str".to_string(),
        "bool" => "boolean".to_string(),
        s if s.starts_with("i") => format!("signed int, {} bits", s[1..].to_string()),
        s if s.starts_with("u") => format!("unsigned int, {} bits", s[1..].to_string()),
        s if s.starts_with("f") => format!("float, {} bits", s[1..].to_string()),
        s if s.starts_with("Vec<") => {
            let inner = type_hint(s[4..s.len() - 1].to_string());
            format!("vector of {}", inner[1..inner.len() - 1].to_string())
        }
        s if s.starts_with("[") && s.ends_with("]") => {
            let type_number = s.split("; ").collect::<Vec<&str>>();
            let inner = type_hint(type_number[0].to_string());
            format!(
                "{}-array of {}",
                type_number[1].to_string(),
                inner[1..inner.len() - 1].to_string()
            )
        }
        s => s.to_string(), // maybe extras for Option<T> or Result<T, E>, ...
    }
    .into();
    format!("[{}{}]", reference, hint)
}

fn attrs_with_type_comment(attrs: &Vec<Attribute>, type_str: &String) -> Vec<Attribute> {
    let mut attrs = attrs.clone();
    let mut path_segments = Punctuated::new();
    path_segments.push(PathSegment {
        ident: format_ident!("doc"),
        arguments: PathArguments::None,
    });
    attrs.insert(
        0,
        Attribute {
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
            bracket_token: Bracket(Span::call_site()),
            meta: Meta::NameValue(MetaNameValue {
                path: Path {
                    leading_colon: None,
                    segments: path_segments,
                },
                eq_token: Token![=](Span::call_site()),
                value: Expr::Lit(ExprLit {
                    attrs: vec![],
                    lit: Lit::Str(LitStr::new(
                        &type_hint(type_str.clone()).as_str(),
                        Span::call_site(),
                    )),
                }),
            }),
        },
    );
    attrs
}

pub fn fields(function: ItemFn) -> Fields {
    let mut fields: Punctuated<Field, Comma> = Punctuated::new();
    function.sig.inputs.iter().for_each(|x| match x {
        FnArg::Typed(PatType {
            attrs,
            pat,
            colon_token,
            ty,
        }) => {
            let type_str = ty.to_token_stream().to_string();
            fields.push(Field {
                attrs: attrs_with_type_comment(&attrs, &type_str),
                vis: Visibility::Inherited,
                mutability: FieldMutability::None,
                ident: match pat.as_ref() {
                    Pat::Ident(PatIdent { ident, .. }) => Some(ident.clone()),
                    _ => None,
                },
                colon_token: Some(*colon_token),
                ty: syn::parse_str(&type_str).unwrap(),
            });
        }
        FnArg::Receiver(_) => {
            panic!("'self' as argument not allowed in function signature.");
        }
    });
    Fields::Named(FieldsNamed {
        brace_token: Brace(Span::call_site()),
        named: fields,
    })
}

pub fn to_struct(name: Ident, fields: Fields) -> Item {
    let mut inner_path = Punctuated::new();
    inner_path.push(PathSegment {
        ident: format_ident!("derive"),
        arguments: PathArguments::None,
    });
    Item::Struct(ItemStruct {
        attrs: vec![Attribute {
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
            bracket_token: Bracket(Span::call_site()),
            meta: Meta::List(MetaList {
                path: Path {
                    leading_colon: None,
                    segments: inner_path,
                },
                delimiter: MacroDelimiter::Paren(Paren(Span::call_site())),
                tokens: quote! { Debug, Clone, Args },
            }),
        }],
        vis: Visibility::Public(Token![pub](Span::call_site())),
        struct_token: Token![struct](Span::call_site()),
        ident: name,
        generics: Generics {
            lt_token: None,
            params: Default::default(),
            gt_token: None,
            where_clause: None,
        },
        fields: fields.clone(),
        semi_token: None,
    })
}

pub fn to_impl_item(new_function: ItemFn) -> ImplItem {
    let function = modify_function(new_function);
    ImplItem::Fn(ImplItemFn {
        attrs: function.attrs,
        vis: function.vis,
        defaultness: None,
        sig: function.sig,
        block: syn::parse_str(&function.block.to_token_stream().to_string()).unwrap(),
    })
}

pub fn modify_function(mut new_function: ItemFn) -> ItemFn {
    new_function.sig.ident = format_ident!("delegate");
    let mut new_function_block = new_function.block.to_token_stream().to_string();
    new_function.sig.inputs.iter().for_each(|x| match x {
        FnArg::Typed(PatType {
            attrs: _,
            pat,
            colon_token: _,
            ty: _,
        }) => {
            let pat = pat.to_token_stream().to_string();
            new_function_block = new_function_block.replace(&pat, &format!("self.{}", pat));
        }
        FnArg::Receiver(_) => {
            panic!("'self' as argument not allowed in function signature.");
        }
    });
    new_function.sig.inputs = Punctuated::new();
    new_function.sig.inputs.push(self_argument());
    new_function.block = syn::parse_str(&new_function_block).unwrap();
    new_function
}

fn self_argument() -> FnArg {
    FnArg::Receiver(Receiver {
        attrs: vec![],
        reference: None,
        mutability: None, // TODO: what about mut arguments?
        self_token: Token![self](Span::call_site()),
        colon_token: Some(Token![:](Span::call_site())),
        ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: format_ident!("Self"),
                    arguments: PathArguments::None,
                }]),
            },
        })),
    })
}
