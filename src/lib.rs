mod module;
mod search;
mod stats;
mod test;

pub mod cli;
mod progress;
mod results;
mod runner;

use pyo3::prelude::*;

#[pymodule]
fn async_test(_: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
