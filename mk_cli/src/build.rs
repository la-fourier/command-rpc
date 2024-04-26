use syn::ItemFn;

pub fn metastruct(item: String) -> std::io::Result<String> {
    todo!()
}

pub mod metastruct {
    pub fn metastruct_name(pre_name: String) -> String {
        pre_name
            .get(0..1)
            .unwrap()
            .to_uppercase()
            .to_string()
            + pre_name.as_str().get(1..).unwrap()
    }

    pub fn metastruct_fields() -> std::io::Result<String> {
        todo!()
    }
}

pub fn convert_function(item: ItemFn) -> String {
    let mut run_function = item.clone();
    run_function.sig.ident = syn::Ident::new("run", run_function.sig.ident.span());
    return String::from("");
}

pub fn impl_from_str() -> std::io::Result<String> {
    todo!()
}
