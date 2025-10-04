use std::path::Path;
use std::{env, fs};
use syn::{File, parse_file};
mod translate;
mod types;
use crate::translate::{print_translated_py_func, translate_pyfn};
use crate::types::PyFunction;

fn read_src(file_path: &Path) -> Option<File> {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let ast: File = parse_file(&contents).expect("Failed to parse file");
    Some(ast)
}

fn extract_fn_name(func: &syn::ItemFn) -> String {
    func.sig.ident.to_string()
}

fn extract_fn_args(func: &syn::ItemFn) -> Vec<(String, String)> {
    let mut args = Vec::new();
    for inp in &func.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = inp {
            let arg_name = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                pat_ident.ident.to_string()
            } else {
                String::new()
            };

            let arg_type = if let syn::Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    seg.ident.to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            args.push((arg_name, arg_type))
        }
    }

    args
}

fn extract_fn_return_type(func: &syn::ItemFn) -> String {
    match &func.sig.output {
        syn::ReturnType::Default => "None".to_string(),
        syn::ReturnType::Type(_, ty) => {
            if let syn::Type::Path(type_path) = &**ty {
                if let Some(seg) = type_path.path.segments.last() {
                    seg.ident.to_string()
                } else {
                    "Any".to_string()
                }
            } else {
                "Any".to_string()
            }
        }
    }
}

fn parse_ast_for_pyfn(ast: &syn::File) -> Vec<PyFunction> {
    let mut pyfunctions: Vec<PyFunction> = Vec::new();

    for item in &ast.items {
        if let syn::Item::Fn(func) = item {
            for attr in &func.attrs {
                if attr.path().is_ident("pyfunction") {
                    let pyfunc = PyFunction {
                        name: extract_fn_name(func),
                        args: extract_fn_args(func),
                        return_type: extract_fn_return_type(func),
                    };
                    pyfunctions.push(pyfunc);
                }
            }
        }
    }
    pyfunctions
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
