use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Formats the sum of two numbers as string.
fn add(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}

#[pymodule]
/// A Python module implemented in Rust.
fn async_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2).unwrap();
        assert_eq!(result, 4);
    }
}
