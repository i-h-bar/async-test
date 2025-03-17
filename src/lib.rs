mod search;
mod stats;
mod module;
mod test;

mod results;
mod runner;

use pyo3::prelude::*;

#[pymodule]
fn async_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
