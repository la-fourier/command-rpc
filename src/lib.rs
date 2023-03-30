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

mod crpc_function;
pub use crpc_function::*;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use regex::Regex;
use std::io::{BufRead, BufReader};

/// Returns an iterator over all Rust source code files in a given directory and its subdirectories.
///
/// # Arguments
///
/// * `dir` - The directory to search for Rust source files.
///
/// # Examples
///
/// ```
/// let files = rust_files_in_dir("path/to/directory").unwrap();
/// ```
pub fn rust_files_in_dir(dir: &str) -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    let path = Path::new(dir);

    Ok(fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .filter(|entry| {
            if let Ok(file_type) = entry.file_type() {
                file_type.is_file() && entry.path().extension().map_or(false, |ext| ext == "rs")
            } else {
                false
            }
        })
        .map(|entry| entry.path()))
}

/// Used to be called in the `build.rs`
fn build(mk_gpt_docs: Option<bool>) -> std::io::Result<()> {
    // Get file, then command iterator
    let path: PathBuf = ["c:\\", "windows", "system32.dll"].iter().collect();
    rust_files_in_dir("./"); // win specific

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


