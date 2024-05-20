pub fn bigger(name: &String) -> String {
    let mut _name = name.clone();
    _name.replace_range(..1, &_name[0..1].to_uppercase());
    return _name;
}

pub fn lower(name: &String) -> String {
    let mut _name = name.clone();
    _name.replace_range(..1, &_name[0..1].to_lowercase());
    return _name;
}

pub mod subcommand {

    use std::path;

    use super::*;

    use quote::*;
    use syn::{
        Item::*,
        __private::Span,
        parse::discouraged::AnyDelimiter,
        punctuated::Punctuated,
        token::{Box, Colon, Comma, Dot, Paren, PathSep, Token},
        *,
    };

    pub fn names(pre_name: String) -> (Ident, Ident) {
        let _name = bigger(&pre_name);
        return (
            format_ident!("{}", _name),
            format_ident!("{}", "Subcommand".to_string() + &_name),
        );
    }

    pub fn subcommands(item: ItemMod) -> Vec<String> {
        let (_, content) = item.content.as_ref().unwrap();
        let mut subcommands: Vec<String> = Vec::new();
        for item in content {
            if let Item::Fn(item) = item {
                for attr in &item.attrs {
                    if (*&attr.meta.path().is_ident("crpc_fn")
                        || *&attr.meta.path().is_ident("crpc"))
                        && AttrStyle::Outer.eq(&attr.style)
                    {
                        subcommands.push(bigger(&item.sig.ident.to_string()));
                    }
                }
            } else if let Item::Mod(item) = item {
                for attr in &item.attrs {
                    if (*&attr.meta.path().is_ident("crpc_mod")
                        || *&attr.meta.path().is_ident("crpc"))
                        && AttrStyle::Outer.eq(&attr.style)
                    {
                        subcommands.push(item.ident.to_string());
                    }
                }
            }
        }
        return subcommands;
    }

    fn sc_to_enum_variant(super_mod_name: &String, subcommand: &String) -> Variant {
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
        // unnamed.push(Field {                Named variant!
        //     attrs: vec![],
        //     vis: Visibility::Inherited,
        //     mutability: FieldMutability::None,
        //     ident: Some(format_ident!("Subcommand{}", bigger(&subcommand))),
        //     colon_token: Some(Token![:](Span::call_site())),
        //     ty: syn::Type::Path(TypePath {
        //         qself: None,
        //         path: Path {
        //             leading_colon: None,
        //             segments: path_segment,
        //         },
        //     }),
        // });

        unnamed.push(Field {
            attrs: vec![],
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
            attrs: vec![],
            ident: format_ident!("Subcommand{}", bigger(&subcommand)),
            fields: Fields::Unnamed(FieldsUnnamed {
                paren_token: Paren::default(),
                unnamed: unnamed,
            }),
            discriminant: None,
        };
    }

    pub fn subcommand_enum(
        super_mod_name: &String,
        subcommands: Vec<String>,
        sc_name: Ident,
    ) -> ItemEnum {
        return ItemEnum {
            attrs: vec![],
            vis: Visibility::Public(Token![pub](Span::call_site())),
            enum_token: Token![enum](Span::call_site()),
            ident: sc_name,
            generics: Generics {
                lt_token: None,
                params: punctuated::Punctuated::new(),
                gt_token: None,
                where_clause: None,
            },
            brace_token: token::Brace(Span::call_site()),
            variants: subcommands
                .iter()
                .map(|subcommand| -> Variant { sc_to_enum_variant(super_mod_name, subcommand) })
                .collect(),
        };
    }

    pub fn delegate_match_arm(
        subcommand: &String,
        enum_name: &Ident,
    ) -> Arm {
        let mut path_segment = Punctuated::new();
        path_segment.push(PathSegment {
            ident: enum_name.clone(),
            arguments: PathArguments::None,
        });
        path_segment.push(PathSegment {
            ident: format_ident!("Subcommand{}", bigger(subcommand)),
            arguments: PathArguments::None,
        });

        let mut path_argument = Punctuated::new();
        path_argument.push(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: Ident::new("command", Span::call_site()),
            subpat: None,
        }));

        let mut body_segment = Punctuated::new();
        body_segment.push(PathSegment {
            ident: Ident::new("command", Span::call_site()),
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
                method: Ident::new("delegate", Span::call_site()),
                turbofish: None,
                paren_token: Paren::default(),
                args: Punctuated::new(),
            })),
            comma: Some(Comma::default()),
        }
    }

    pub fn _delegate_match_arm(super_mod_name: &String, subcommand: &String) -> Arm {
        let mut seg: Punctuated<PathSegment, PathSep> = Punctuated::new(); // maybe need to do TestCommand(command) other way..
        seg.push(PathSegment {
            ident: format_ident!("{}::{}", lower(super_mod_name), bigger(&subcommand)), /*Ident::new(
                                                                                            format!("{}::{}", lower(super_mod_name), bigger(&subcommand)).as_str(),
                                                                                            Span::call_site(),
                                                                                        ),*/
            arguments: PathArguments::None,
        });
        let mut elems_: Punctuated<Pat, _> = Punctuated::new();
        elems_.push(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: format_ident!("command"),
            subpat: None,
        }));
        Arm {
            attrs: vec![],
            pat: Pat::TupleStruct(PatTupleStruct {
                attrs: vec![],
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: seg,
                },
                paren_token: Paren(Span::call_site()),
                elems: elems_,
            }),
            guard: None,
            fat_arrow_token: Token![=>](Span::call_site()),
            body: std::boxed::Box::new(Expr::Verbatim(quote! {command.delegate()})),
            comma: Some(Token![,](Span::call_site())),
        }
    }

    pub fn delegate_match_expr(
        super_mod_name: &String,
        subcommands: Vec<String>,
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
                .map(|subcommand| -> Arm {
                    delegate_match_arm(subcommand, enum_name)
                })
                .collect(),
        }
    }
}

pub mod command_args {
    use quote::*;
    use syn::{__private::Span, punctuated::Punctuated, *, token::Comma};

    pub fn fields(function: ItemFn) -> Fields {
        let mut fields: Punctuated<Field, Comma> = Punctuated::new();
        function.sig.inputs.iter().for_each(|x|  match x {
            FnArg::Typed(PatType {
                attrs,
                pat,
                colon_token,
                ty,
            }) => {
                fields.push(Field {
                    attrs: attrs.clone(),
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: match pat.as_ref() {
                        Pat::Ident(PatIdent { ident, .. }) => Some(ident.clone()),
                        _ => None,
                    },
                    colon_token: Some(*colon_token),
                    ty: syn::parse_str(&ty.to_token_stream().to_string()).unwrap(),
                });
            }
            FnArg::Receiver(_) => {
                panic!("'self' as argument not allowed in function signature.");
            }
            _ => {}
        });
        Fields::Named(FieldsNamed {
            brace_token: token::Brace(Span::call_site()),
            named: fields,
        })
    }

    pub fn modify_function(mut new_function: ItemFn) -> ItemFn {
        new_function.sig.ident = format_ident!("delegate");
        new_function.attrs = vec![];
        new_function.sig.output = parse_quote!(Option<String>);
        let mut new_function_block = new_function.block.to_token_stream().to_string();
        new_function.sig.inputs.iter().for_each(|x| match x {
            FnArg::Typed(PatType {
                attrs,
                pat,
                colon_token,
                ty,
            }) => {
                let pat = pat.to_token_stream().to_string();
                new_function_block = new_function_block.replace(&pat, &format!("self.{}", pat));
            }
            FnArg::Receiver(_) => {
                panic!("'self' as argument not allowed in function signature.");
            }
            _ => {}
        });
        new_function.sig.inputs = Punctuated::new();
        new_function
    }

    pub fn item_call(ident: Ident) -> ExprCall {
        let mut path = Punctuated::new();
        path.push(PathSegment {
            ident: format_ident!("{}", ident),
            arguments: PathArguments::None,
        });
        ExprCall {
            attrs: vec![],
            func: std::boxed::Box::new(Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: path,
                },
            })),
            args: Punctuated::new(),
            paren_token: token::Paren(Span::call_site()),
        }
    }
}
