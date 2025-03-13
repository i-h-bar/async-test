mod loader;
mod search;
mod stats;

use crate::search::Tests;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn main(path: String) -> PyResult<Vec<String>> {
    let tests = Tests::find(path);
    Ok(tests.tests().to_owned())
}

#[pymodule]
fn async_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;

    Ok(())
}
