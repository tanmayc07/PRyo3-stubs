#[derive(Debug, Clone)]
pub struct PyFunction {
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,
}

#[derive(Debug, Clone)]
pub struct PyStubFunction {
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,
}
