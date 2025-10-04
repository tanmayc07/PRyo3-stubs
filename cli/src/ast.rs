use crate::types::PyFunction;

pub fn extract_fn_name(func: &syn::ItemFn) -> String {
    func.sig.ident.to_string()
}

pub fn extract_fn_args(func: &syn::ItemFn) -> Vec<(String, String)> {
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

pub fn extract_fn_return_type(func: &syn::ItemFn) -> String {
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

pub fn parse_ast_for_pyfn(ast: &syn::File) -> Vec<PyFunction> {
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