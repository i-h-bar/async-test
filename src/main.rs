use pyo3::PyResult;
use async_test::cli;

#[pyo3_async_runtimes::tokio::main]
async fn main() -> PyResult<()> {
    cli::run().await
}
