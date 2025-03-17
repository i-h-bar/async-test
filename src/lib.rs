mod module;
mod search;
mod stats;
mod test;

mod results;
mod runner;
pub mod cli;

use pyo3::prelude::*;

#[pymodule]
fn async_test(_: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
