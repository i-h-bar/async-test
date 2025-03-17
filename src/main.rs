use crate::stats::Stats;
use crate::test::{modularise, run_module};
use futures::lock::Mutex;
use indicatif::MultiProgress;
use pyo3::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

pub mod results;
mod search;
mod stats;
mod test;

#[pyo3_async_runtimes::tokio::main]
async fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let _ = py
            .import("sys")
            .expect("sys module not found")
            .getattr("path")
            .expect("path attribute not found in sys module")
            .getattr("append")
            .expect("append method not found for list")
            .call1(("./",));
    });

    let tests: Vec<String> = search::async_search(PathBuf::from("./"))
        .await
        .into_iter()
        .filter_map(|path| modularise(path).ok())
        .collect();

    let longest_name = tests
        .iter()
        .map(|module| module.split(".").last().unwrap().len())
        .max()
        .unwrap();

    let stats = Arc::new(Mutex::new(Stats::new(tests.len())));

    let multi_bar = MultiProgress::new();

    futures::future::try_join_all(
        tests
            .into_iter()
            .map(|test| run_module(test, Arc::clone(&stats), &multi_bar, longest_name)),
    )
    .await?;
    Ok(())
}
