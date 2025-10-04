use crate::types::{PyFunction, PyStubFunction};
use std::io::{self, Write};

pub fn translate_pyfn(py_func: &PyFunction) -> PyStubFunction {
    let mut translated_args: Vec<(String, String)> = Vec::new();

    for arg in &py_func.args {
        let arg_name = arg.0.clone();

        let arg_type = match arg.1.as_str() {
            "i32" | "u32" => "int",
            "f32" | "f64" => "float",
            "bool" => "bool",
            "char" | "String" | "&str" => "str",
            "()" => "None",
            _ => "Any",
        }
        .to_string();

        translated_args.push((arg_name, arg_type));
    }

    let ret_type = match py_func.return_type.as_str() {
        "i32" | "u32" => "int",
        "f32" | "f64" => "float",
        "bool" => "bool",
        "char" | "String" | "&str" => "str",
        "()" => "None",
        _ => "Any",
    }
    .to_string();

    let py_stub_fn = PyStubFunction {
        name: py_func.name.clone(),
        args: translated_args,
        return_type: ret_type,
    };

    py_stub_fn
}

pub fn print_translated_py_func(py_func: &PyStubFunction) {
    print!("def {}(", py_func.name);

    let mut args_len = py_func.args.len();
    for arg in &py_func.args {
        print!("{}: {}", arg.0.as_str(), arg.1.as_str());
        args_len -= 1;
        if args_len > 0 {
            print!(", ");
        }
    }

    print!(") -> {}: ...\n", py_func.return_type.as_str());
    io::stdout().flush().unwrap();
}
