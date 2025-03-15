use std::ops::Deref;
use crate::stats::Stats;
use crate::test::run_test;
use futures::lock::Mutex;
use pyo3::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

mod search;
mod stats;
mod test;
pub mod results;

#[pyo3_async_runtimes::tokio::main]
async fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let _ = py
            .import("sys")
            .expect("sys module not found")
            .getattr("path").expect("path attribute not found in sys module")
            .getattr("append")
            .expect("append method not found for list")
            .call1(("./",));
    });

    let tests = search::async_search(PathBuf::from("./")).await;
    let stats = Arc::new(Mutex::new(Stats::new(tests.len())));

    futures::future::try_join_all(tests.into_iter().map(|test| run_test(test, Arc::clone(&stats)))).await?;

    println!("{:#?}", stats.lock().await.deref());

    Ok(())
}
