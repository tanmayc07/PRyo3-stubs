use std::path::Path;
use std::{env, fs};
use syn::{File, parse_file};

mod ast;
mod types;
mod translate;

use crate::ast::{parse_ast_for_pyfn};
use crate::translate::{print_translated_py_func, translate_pyfn};

fn read_src(file_path: &Path) -> Option<File> {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let ast: File = parse_file(&contents).expect("Failed to parse file");
    Some(ast)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cli <path-to-file>");
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let ast = read_src(&file_path).unwrap();

    let py_functions = parse_ast_for_pyfn(&ast);
    println!("{:#?}", &py_functions[0]);

    let translated_py_fn = translate_pyfn(&py_functions[0]);
    println!("{:#?}", &translated_py_fn);
    print_translated_py_func(&translated_py_fn);
}

// #[cfg(test)]
// mod tests {
//     use super::*;


// }
