use async_test::cli;
use pyo3::PyResult;

#[pyo3_async_runtimes::tokio::main]
async fn main() -> PyResult<()> {
    cli::run().await
}
