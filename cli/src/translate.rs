use crate::types::{PyFunction, PyStubFunction};
use std::io::{self, Write};

impl PyFunction {
    pub fn translate(&self) -> PyStubFunction {
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

impl PyStubFunction {
    pub fn to_stub(&self) -> String {
        let mut args_str = String::new();
        for (i, (name, typ)) in self.args.iter().enumerate() {
            args_str.push_str(&format!("{}: {}", name, typ));
            if i < self.args.len() - 1 {
                args_str.push_str(", ");
            }
        }
        format!("def {}({}) -> {}: ...\n", self.name, args_str, self.return_type)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{PyFunction, PyStubFunction};

    #[test]
    fn test_translate_basic_types() {
        let pyfn = PyFunction {
            name: "foo".to_string(),
            args: vec![
                ("a".to_string(), "i32".to_string()),
                ("b".to_string(), "String".to_string())
            ],
            return_type: "bool".to_string(),
        };
        let stub = pyfn.translate();
        assert_eq!(stub.args, vec![
            ("a".to_string(), "int".to_string()),
            ("b".to_string(), "str".to_string())
        ]);
        assert_eq!(stub.return_type, "bool");
    }

    #[test]
    fn test_translate_unknown_type() {
        let pyfn = PyFunction {
            name: "bar".to_string(),
            args: vec![("x".to_string(), "MyType".to_string())],
            return_type: "MyType".to_string()
        };
        let stub = pyfn.translate();
        assert_eq!(stub.args[0].1, "Any");
        assert_eq!(stub.return_type, "Any");
    }
}