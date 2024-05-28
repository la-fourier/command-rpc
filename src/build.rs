use quote::format_ident;
use syn::{
    Item, ItemUse, Token, Visibility, __private::Span, punctuated::Punctuated, token::Brace,
    UseGroup, UseName, UsePath, UseTree,
};

/// Returns the first letter of the string in uppercase
pub fn bigger(name: &String) -> String {
    let mut _name = name.clone();
    _name.replace_range(..1, &_name[0..1].to_uppercase());
    return _name;
}

/// Returns the first letter of the string in lowercase
pub fn lower(name: &String) -> String {
    let mut _name = name.clone();
    _name.replace_range(..1, &_name[0..1].to_lowercase());
    return _name;
}

/// Returns just "use clap::{Args, Parser, Subcommand};" as AST item
pub fn item_use() -> Item {
    let mut items = Punctuated::new();
    items.push(UseTree::Name(UseName {
        ident: format_ident!("Args"),
    }));
    items.push(UseTree::Name(UseName {
        ident: format_ident!("Parser"),
    }));
    items.push(UseTree::Name(UseName {
        ident: format_ident!("Subcommand"),
    }));
    Item::Use(ItemUse {
        attrs: vec![],
        vis: Visibility::Inherited,
        leading_colon: None,
        use_token: Token![use](Span::call_site()),
        tree: UseTree::Path(UsePath {
            ident: format_ident!("clap"),
            colon2_token: Token![::](Span::call_site()),
            tree: std::boxed::Box::new(UseTree::Group(UseGroup {
                brace_token: Brace(Span::call_site()),
                items: items,
            })),
        }),
        semi_token: Token![;](Span::call_site()),
    })
}
