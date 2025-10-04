use crate::types::{PyFunction, PyStubFunction};
use std::io::{self, Write};

impl PyFunction {
    pub fn to_stub(&self) -> PyStubFunction {
        let translated_args = self.args.iter().map(|(name, ty)| {
            let py_type = match ty.as_str() {
                "i32"|"u32"|"usize"|"isize" => "int",
                "f32"|"f64" => "float",
                "bool" => "bool",
                "char"|"String"|"&str" => "str",
                "()" => "None",
                _ => "Any",
            }.to_string();
            (name.clone(), py_type)
        }).collect();

        let ret_type = match self.return_type.as_str() {
            "i32"|"u32"|"usize"|"isize" => "int",
            "f32"|"f64" => "float",
            "bool" => "bool",
            "char"|"String"|"&str" => "str",
            "()" => "None",
            _ => "Any",
        }.to_string();

        PyStubFunction { name:  self.name.clone(), args: translated_args, return_type: ret_type }
    }
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
