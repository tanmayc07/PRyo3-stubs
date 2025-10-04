#[derive(Debug)]
pub struct PyFunction {
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,
}

#[derive(Debug)]
pub struct PyStubFunction {
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,
}
