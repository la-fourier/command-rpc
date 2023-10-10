use comment_macro::extract_comments;

fn main() {
    println!("Hello, world!");
    func("Hello, world!", 10);
}

#[extract_comments]
// This is a comment outside of the module with two slashes
/// This is a doc comment outside of the module
//! This is a doc comment outside of the module with two slashes and an exclamation mark
fn main(
    d: &str, // Comment of the first argument d, a stirng
    // This is a comment inside the function with two slashes - has its own line
    /* inline cimment before i definition*/ i: i32, 
) {
    // This is a comment inside the module with two slashes
    println!("Hello, world!");
    // After some command we have a comment that is the las one
}
