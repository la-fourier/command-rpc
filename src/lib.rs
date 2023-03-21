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

fn build(mk_gpt_docs: Option<bool>) {
    // Get file, then command iterator

    // Check for docs
    match mk_gpt_docs {
        Some(true) => {},
        _ => {}
    }

    // Generate Cli app

    // Generate parse structure of cli app
        // insert function value giving
}


fn mk_cli() {
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

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);

    // run the script and get a handle to the running child process
    let child = run_script::spawn(
        r#"
         echo "Directory Info:"
         dir
         "#,
        &args,
        &options,
    )
    .unwrap();

    let spawn_output = child.wait_with_output().unwrap();

    println!("Success: {}", &spawn_output.status.success());
}