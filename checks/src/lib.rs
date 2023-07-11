pub fn check_fromstr() {
    let s = "123";
    let n = s.parse::<i32>().unwrap();
    assert_eq!(n, 123);
}

pub mod type_checks {
    pub fn check_fromstr() {
        let s = "123";
        let n = s.parse::<i32>().unwrap();
        assert_eq!(n, 123);
    }

    pub fn output_check(item: &syn::ItemFn) {
        if let syn::ReturnType::Type(_, boxed) = item.sig.output.clone() {
            let type_name = boxed.to_token_stream();
            
            if let syn::Type::Path(path) = *boxed.clone() {
                if let Some(ident) = path.path.get_ident() {
                    if ident.to_string() == "String" {
                        println!("Your cli returns a String. This is ok but might cause speed issues.");
                    }
                }
                if match &*boxed {
                    syn::Type::Path(type_path) => {
                        if let Some(segment) = type_path.path.segments.last() {
                            segment.ident.to_string() == String::from("std::str::ToStr") // TODO fix! Or it shall be something convertable to a str
                        } else {
                            false
                        }
                    }
                    _ => false,
                } {

                };
            }
        }
    }
}