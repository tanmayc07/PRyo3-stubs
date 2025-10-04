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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_extract_fn_name() {
        let func: syn::ItemFn = parse_quote! {fn foo(x: i32) -> i32 {x} };
        assert_eq!(extract_fn_name(&func), "foo");
    }

    #[test]
    fn test_extract_args_with_args() {
        let func: syn::ItemFn = parse_quote! {fn foo(x: i32, y: f64) -> i32 {x}};
        assert_eq!(
            extract_fn_args(&func),
            vec![
                ("x".to_string(), "i32".to_string()),
                ("y".to_string(), "f64".to_string())
            ]
        );
    }

    #[test]
    fn test_extract_args_without_args() {
        let func: syn::ItemFn = parse_quote! {fn foo() -> i32 {x}};
        assert_eq!(
            extract_fn_args(&func),
            vec![]
        )
    }

    #[test]
    fn test_extract_fn_return_ty_with_ret_ty() {
        let func: syn::ItemFn = parse_quote! {fn foo(x: i32, y:f64) -> Vec<String> {x}};
        assert_eq!(
            extract_fn_return_type(&func),
            "Vec"
        );
    }

    #[test]
    fn test_extract_fn_return_ty_without_ret_ty() {
        let func: syn::ItemFn = parse_quote! {fn foo(x: i32, y:f64) {x}};
        assert_eq!(
            extract_fn_return_type(&func),
            "None"
        );
    }

    #[test]
    fn test_extract_fn_return_ty_with_complex_ret_ty() {
        let func: syn::ItemFn = parse_quote! {fn foo() -> (i32, f64) {x}};
        assert_eq!(
            extract_fn_return_type(&func),
            "Any"
        );
    }

    #[test]
    fn test_parse_ast_for_pyfn() {
        let ast: syn::File = syn::parse_quote! {
            #[pyfunction]
            fn foo(x: i32, y: f64) -> i32 {x}

            fn bar(y: bool) -> f64 {y}

            #[pyfunction]
            fn baz() -> bool {true}
        };

        let pyfns = parse_ast_for_pyfn(&ast);
        assert_eq!(pyfns.len(), 2);
        assert_eq!(pyfns[0].name, "foo");
        assert_eq!(pyfns[1].name, "baz");
        assert_eq!(
            pyfns[0].args, vec![
                ("x".to_string(), "i32".to_string()),
                ("y".to_string(), "f64".to_string())
            ]
        );
        assert_eq!(pyfns[1].args, vec![]);
        assert_eq!(pyfns[0].return_type, "i32");
        assert_eq!(pyfns[1].return_type, "bool");
    }
}