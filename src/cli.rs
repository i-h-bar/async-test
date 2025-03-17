use std::ops::Deref;
use std::path::PathBuf;
use std::time::Instant;
use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use crate::module::{modularise, run_module};
use crate::runner::SuiteRunner;
use crate::search;

pub async fn run() -> PyResult<()> {
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

    let start = Instant::now();

    let tests: Vec<String> = search::async_search(PathBuf::from("./"))
        .await
        .into_iter()
        .filter_map(|path| modularise(path).ok())
        .collect();

    let suite = SuiteRunner::new();

    futures::future::try_join_all(tests.into_iter().map(|test| run_module(test, &suite))).await?;

    println!(
        "\n\nFinished in {:?}:{}",
        start.elapsed(),
        suite.stats().await.deref()
    );
    Ok(())
}