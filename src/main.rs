use crate::loader::Test;
use crate::stats::Stats;
use pyo3::prelude::*;

mod loader;
mod stats;

#[pyo3_async_runtimes::tokio::main]
async fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let syspath = py.import("sys").unwrap().getattr("path").unwrap();

        syspath.call_method1("append", ("./",)).unwrap();
    });

    let stats = Stats::new();
    let test = Test::new("mock_tests.test_file", stats)?;
    test.run().await?;

    Ok(())
}
