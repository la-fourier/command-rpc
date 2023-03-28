use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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

use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct CrpcFunction {
    pub source: String,
    pub module_path: String,
    pub documentation: String,
    pub parameters: String,
}

impl CrpcFunction {
    pub fn new(source: String, module_path: String) -> CrpcFunction {
        CrpcFunction {
            source,
            module_path,
            documentation: String::new(),
            parameters: String::new(),
        }
    }

    /// Parses a Rust source file and returns a vector of `CrpcFunction` structs representing all functions
    /// marked with the `#[crpc(core)]` or `#[crpc(sub)]` attribute.
    ///
    /// # Arguments
    ///
    /// * `file_name` - A `&str` that holds the name of the Rust source file.
    ///
    /// # Examples
    ///
    /// ```
    /// use crpc_generator::get_all_crpc_functions;
    ///
    /// let file_name = "tests/test_file.rs";
    /// let result = get_all_crpc_functions(file_name);
    ///
    /// assert!(result.is_ok());
    /// ```
    pub fn get_all_crpc_functions(file_name: &str) -> Result<Vec<CrpcFunction>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();
        let file = fs::File::open(file_name)?;
        let reader = BufReader::new(file);
        let mut current_function = None;

        // Regular expressions to match the `#[crpc]` attribute, function documentation, and function parameters.
        let crpc_attribute_re = Regex::new(r#"^\s*#\[(crpc\(core\)|crpc\(sub\))\]\s*$"#)?;
        let doc_re = Regex::new(r#"^\s*///(.*)$"#)?;
        let param_re = Regex::new(r#"^\s*([^:]+):\s*([^,]+),?\s*$"#)?;

        for line_result in reader.lines() {
            let line = line_result?;

            if let Some(ref mut function) = current_function {
                // If we're currently processing a function, check if we've reached its end.
                if line.contains('}') {
                    if !function.documentation.is_empty() {
                        // Remove the "/// " from the start of each line of documentation.
                        function.documentation = function
                            .documentation
                            .lines()
                            .map(|line| line.trim_start_matches("/// ").to_owned())
                            .collect::<Vec<_>>()
                            .join("\n");
                    }
                    result.push(function.clone());
                    current_function = None;
                } else if let Some(cap) = param_re.captures(&line) {
                    // If the line matches the function parameter pattern, add it to the current function's
                    // list of parameters.
                    let name = cap.get(1).unwrap().as_str().to_owned();
                    let ty = cap.get(2).unwrap().as_str().to_owned();
                    function.parameters.push_str(&format!("{},{};", name, ty));
                } else if let Some(cap) = doc_re.captures(&line) {
                    // If the line matches the function documentation pattern, add it to the current
                    // function's documentation.
                    let doc = cap.get(1).unwrap().as_str().to_owned();
                    function.documentation.push_str(&format!("{}\n", doc));
                }
            } else if let Some(cap) = crpc_attribute_re.captures(&line) {
                // If the line matches the `#[crpc]` attribute pattern, create a new `CrpcFunction`
                // and store its source code and module path.
            }
        }
    }
}