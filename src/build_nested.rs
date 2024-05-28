/// All functions used for crpc_main and crpc_mod bc they are basically the same except
/// implementation of "Parser" and "Subcommand". Everything defined here is used in both.
/// The name comes from the fact that the functions are used for the subcommands of the main command with
/// nested subcommands.
use crate::build::{bigger, lower};
use quote::format_ident;
use syn::{
    AttrStyle, Field, FieldMutability, Fields, FieldsUnnamed, Path, PathArguments, PathSegment,
    TypePath, Visibility,
    __private::Span,
    punctuated::Punctuated,
    token::{Brace, Comma, Dot, Paren, PathSep},
    Arm, Attribute, Expr, ExprLit, ExprMatch, ExprMethodCall, ExprPath, Generics, Ident, Item,
    ItemEnum, ItemFn, ItemMod, Lit, LitStr, Meta, MetaNameValue, Pat, PatIdent, PatTupleStruct,
    Token, parse_quote, Variant,
};

fn delete_same(attrs: Vec<Attribute>) -> Vec<Attribute> { // Needed for unknown reasons, everything appears twice
    let mut new_attrs = Vec::new();
    let mut same = false;
    attrs.iter().for_each(|attr| {
        if !new_attrs.contains(attr) {
            new_attrs.push(attr.clone());
        }
    });
    return new_attrs;
}

pub fn names(pre_name: String) -> (Ident, Ident) {
    let _name = bigger(&pre_name);
    return (
        format_ident!("{}", _name),
        format_ident!("{}", "Subcommand".to_string() + &_name),
    );
}

#[deprecated]
pub fn subcommands(item: ItemMod) -> Vec<String> {
    let (_, content) = item.content.as_ref().unwrap();
    let mut subcommands: Vec<String> = Vec::new();
    for item in content {
        if let Item::Fn(item) = item {
            for attr in &item.attrs {
                if (*&attr.meta.path().is_ident("crpc_fn") || *&attr.meta.path().is_ident("crpc"))
                    && AttrStyle::Outer.eq(&attr.style)
                {
                    subcommands.push(bigger(&item.sig.ident.to_string()));
                }
            }
        } else if let Item::Mod(item) = item {
            for attr in &item.attrs {
                if (*&attr.meta.path().is_ident("crpc_mod") || *&attr.meta.path().is_ident("crpc"))
                    && AttrStyle::Outer.eq(&attr.style)
                {
                    subcommands.push(format!("{}_", item.ident));
                }
            }
        }
    }
    return subcommands;
}

pub trait FnMod {
    fn ident(&self) -> String;
    fn attrs(&self) -> &Vec<Attribute>;
}

impl FnMod for ItemMod {
    fn ident(&self) -> String {
        let s = self.ident.clone().to_string();
        format!("{}", s)
    }
    fn attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}
impl FnMod for ItemFn {
    fn ident(&self) -> String {
        format!("{}", self.sig.ident.clone().to_string())
    }
    fn attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}

pub fn subcommand_with_help(item: impl FnMod) -> (String, Vec<Attribute>) {
    // TODO bigger refactor..
    let name = bigger(&item.ident());
    let mut comments = Vec::new();
    let mut path_segments: Punctuated<PathSegment, PathSep> = Punctuated::new();
    path_segments.push(PathSegment {
        ident: format_ident!("doc"),
        arguments: PathArguments::None,
    });
    item.attrs().iter().for_each(|attr| {
        if let Attribute {
            pound_token: _,
            style: AttrStyle::Outer,
            bracket_token: _,
            meta:
                Meta::NameValue(MetaNameValue {
                    path:
                        Path {
                            leading_colon: None,
                            segments: punctuated,
                        },
                    eq_token: _,
                    value:
                        Expr::Lit(ExprLit {
                            attrs: _,
                            lit: Lit::Str(LitStr { .. }),
                        }),
                }),
        } = attr
        {
            if punctuated.len() == 1
                && punctuated.first().unwrap()
                    == &(PathSegment {
                        ident: format_ident!("doc"),
                        arguments: PathArguments::None,
                    })
            {
                comments.push(attr.clone());
            }
            comments.push(attr.clone());
        }
    });
    return (name, comments);
}

pub fn subcommands_with_help(item: ItemMod) -> Vec<(String, Vec<Attribute>)> {
    let (_, content) = item.content.as_ref().unwrap();
    let mut subcommands: Vec<(String, Vec<Attribute>)> = Vec::new();
    for item in content {
        if let Item::Fn(item) = item {
            for attr in &item.attrs {
                if (*&attr.meta.path().is_ident("crpc_fn") || *&attr.meta.path().is_ident("crpc"))
                    && AttrStyle::Outer.eq(&attr.style)
                {
                    subcommands.push(subcommand_with_help(item.clone()));
                }
            }
        } else if let Item::Mod(item) = item {
            for attr in &item.attrs {
                if (*&attr.meta.path().is_ident("crpc_mod") || *&attr.meta.path().is_ident("crpc"))
                    && AttrStyle::Outer.eq(&attr.style)
                {
                    subcommands.push(subcommand_with_help(item.clone()));
                }
            }
        }
    }
    return subcommands;
}

fn sc_to_enum_variant(
    super_mod_name: &String,
    subcommand: &String,
    attrs: Vec<Attribute>,
) -> Variant {
    let mut path_segment = Punctuated::new();
    path_segment.push(PathSegment {
        ident: format_ident!("{}", lower(super_mod_name)),
        arguments: PathArguments::None,
    });
    path_segment.push(PathSegment {
        ident: format_ident!("{}", bigger(&subcommand)),
        arguments: PathArguments::None,
    });
    let mut unnamed: Punctuated<Field, syn::token::Comma> = Punctuated::new();
    unnamed.push(Field {
        attrs: vec![], // TODO for clap interaction: auch von mod?
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: None,
        colon_token: None,
        ty: syn::Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: path_segment,
            },
        }),
    });
    return Variant {
        attrs: delete_same(attrs),
        ident: format_ident!("{}", bigger(&subcommand)),
        fields: Fields::Unnamed(FieldsUnnamed {
            paren_token: Paren::default(),
            unnamed: unnamed,
        }),
        discriminant: None,
    };
}

pub fn subcommand_enum(
    super_mod_name: &String,
    subcommands: Vec<(String, Vec<Attribute>)>,
    sc_name: Ident,
) -> ItemEnum {
    return ItemEnum {
        attrs: vec![],
        vis: Visibility::Public(Token![pub](Span::call_site())),
        enum_token: Token![enum](Span::call_site()),
        ident: sc_name,
        generics: Generics {
            lt_token: None,
            params: Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        brace_token: Brace(Span::call_site()),
        variants: subcommands
            .iter()
            .map(|(subcommand, attrs)| -> Variant {
                sc_to_enum_variant(super_mod_name, subcommand, attrs.to_vec())
            })
            .collect(),
    };
}

pub fn delegate_match_arm(subcommand: &String, enum_name: &Ident) -> Arm {
    let mut path_segment = Punctuated::new();
    path_segment.push(PathSegment {
        ident: enum_name.clone(),
        arguments: PathArguments::None,
    });
    path_segment.push(PathSegment {
        ident: format_ident!("{}", bigger(subcommand)),
        arguments: PathArguments::None,
    });
    let mut path_argument = Punctuated::new();
    path_argument.push(Pat::Ident(PatIdent {
        attrs: vec![],
        by_ref: None,
        mutability: None,
        ident: format_ident!("command"),
        subpat: None,
    }));
    let mut body_segment = Punctuated::new();
    body_segment.push(PathSegment {
        ident: format_ident!("command"),
        arguments: PathArguments::None,
    });
    Arm {
        attrs: vec![],
        pat: Pat::TupleStruct(PatTupleStruct {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: path_segment,
            },
            paren_token: Paren::default(),
            elems: path_argument,
        }),
        guard: None,
        fat_arrow_token: Token![=>](Span::call_site()),
        body: std::boxed::Box::new(Expr::MethodCall(ExprMethodCall {
            attrs: vec![],
            receiver: std::boxed::Box::new(Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: body_segment,
                },
            })),
            dot_token: Dot::default(),
            method: format_ident!("delegate"),
            turbofish: None,
            paren_token: Paren::default(),
            args: Punctuated::new(),
        })),
        comma: Some(Comma::default()),
    }
}

pub fn delegate_match_expr(
    subcommands: Vec<(String, Vec<Attribute>)>,
    enum_name: &Ident,
) -> ExprMatch {
    ExprMatch {
        attrs: vec![],
        match_token: Token![match](Span::call_site()),
        expr: std::boxed::Box::from(Expr::Path(ExprPath {
            attrs: vec![],
            qself: None,
            path: Path::from(format_ident!("self")),
        })),
        brace_token: syn::token::Brace::default(),
        arms: subcommands
            .iter()
            .map(|(subcommand, _)| -> Arm { delegate_match_arm(subcommand, enum_name) })
            .collect(),
    }
}