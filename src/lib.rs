use pyo3::{pyfunction, pymodule, wrap_pyfunction, PyResult, Python};
use pyo3::types::PyModule;

#[pyfunction]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[pymodule]
fn async_test(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
