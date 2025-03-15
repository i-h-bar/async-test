use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::test::Test;
use crate::stats::Stats;
use pyo3::prelude::*;

mod test;
mod stats;
mod search;

#[pyo3_async_runtimes::tokio::main]
async fn main() -> PyResult<()> {
    let tests = search::async_search(PathBuf::from("./")).await;

    println!("{:#?}", tests.len());

    Ok(())
}
