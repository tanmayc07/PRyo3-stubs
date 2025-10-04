use std::path::Path;
use std::{env, fs};
use syn::{File, parse_file};

mod ast;
mod translate;
mod types;

use crate::ast::parse_ast_for_pyfn;
use crate::translate::print_translated_py_func;

fn read_src(file_path: &Path) -> Result<File, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let ast: File = parse_file(&contents)?;
    Ok(ast)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cli <path-to-file>");
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    if !file_path.exists() {
        eprintln!("File does not exist: {:?}", file_path);
        std::process::exit(1);
    }
    if !file_path.is_file() {
        eprintln!("Path is not a file: {:?}", file_path);
        std::process::exit(1);
    }

    let ast =
        read_src(&file_path).expect(&format!("Failed to read or parse file: {:?}", file_path));

    let py_functions = parse_ast_for_pyfn(&ast);
    println!("{:#?}", &py_functions[0]);

    let translated_py_fn = py_functions[0].to_stub();
    println!("{:#?}", &translated_py_fn);
    print_translated_py_func(&translated_py_fn);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_read_src_nonexistent_file() {
        let result = read_src(Path::new("nonexistent_file.rs"));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_src_invalid_rust() {
        let path = Path::new("test_invalid.rs");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "not valid rust code").unwrap();

        let result = read_src(&path);
        assert!(result.is_err());

        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_read_src_valid_rust() {
        let path = Path::new("test_valid.rs");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "fn foo() {{}}").unwrap();

        let result = read_src(&path);
        assert!(result.is_ok());

        std::fs::remove_file(&path).unwrap();
    }
}
