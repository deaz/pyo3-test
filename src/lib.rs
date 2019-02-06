use pyo3::prelude::*;

#[pyclass]
struct Class {}

#[pymethods]
impl Class {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|| Class {})
    }
}

#[pymodule]
fn pyo3_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Class>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
