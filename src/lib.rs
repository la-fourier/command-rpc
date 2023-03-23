/*pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

use run_script::*;

extern crate build;
use build::*;

/// Used to be called in the `build.rs`
fn build(mk_gpt_docs: Option<bool>) -> std::io::Result<()> {
    // Get file, then command iterator
    for file in get_file_iterator("./") {

    }

    // Check for docs
    match mk_gpt_docs {
        Some(true) => {},
        _ => {}
    }

    // Generate Cli app

    // Generate parse structure of cli app
        // insert function value giving

    Ok(())
}


fn get_file_iterator(path: String) -> Vec<String> {
    let home = std::env::current_dir();
    std::env::cange_dir(path);
    let options = ScriptOptions::new();
    let args = vec![];
    // run the script and get the script execution output
    let (code, output, error) = run_script::run(
        r#"
         tree
         "#,
        &args,
        &options,
    )
    .unwrap();

    println!("Scanning files: {}, {}", code, error);

    for raw in output.split("\n") {
        todo!()
    }

    std::env::cange_dir(home);
}

fn crop_tree_result(s: String) -> String {
    todo!()
}