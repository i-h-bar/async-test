mod search;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use crate::search::Tests;

#[pyfunction]
fn main(path: String) -> PyResult<Vec<String>> {
    let tests = Tests::find(path);
    Ok(tests.tests().to_owned())
}

#[pymodule]
/// A Python module implemented in Rust.
fn async_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;

    Ok(())
}
