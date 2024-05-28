/// Process information given in brackets to the proc macros defined in lib
/// This will need better integration in the proc macros to be useful because
/// the code will have to be inserted at diffrent points.

macro_rules! process {
    ($name:ident, $attr:expr) => {
        $name($attr);
    };
}

pub fn parse(attr: TokenStream) {
    let attr = attr.to_string().replace(" ", "");
    attr.split(",").collect::<Vec<&str>>().iter().for_each(|attr| {
        let attr = attr.split("=").collect::<Vec<&str>>();
        let name = attr[0];
        let value = attr[1];
        process!(name, value);
    });
}