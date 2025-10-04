use pyo3::prelude::*;

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pyclass]
struct Person {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    age: u32,
}

#[pymodule]
fn my_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<Person>()?;
    Ok(())
}