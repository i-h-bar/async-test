mod search;
mod stats;
mod test;

use pyo3::prelude::*;


#[pymodule]
fn async_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
