
use std::future::Future;
use crate::stats::Stats;
use pyo3::prelude::*;
use pyo3::{PyResult, Python};

pub struct Test {
    test: Box<dyn Future<Output=PyResult<PyObject>> + Send + 'static>,
    stats: Stats,
}

impl Test {
    pub fn new(path: &str, stats: Stats) -> PyResult<Self> {
        let test = Python::with_gil(|py| {
            let asyncio = py.import(path)?;
            pyo3_async_runtimes::tokio::into_future(asyncio.call_method0("test_case")?)
        })?;

        Ok(Self {
            test: Box::new(test),
            stats,
        })
    }

    pub async fn run(self) -> PyResult<()> {
        self.test.await?;

        Ok(())
    }
}
