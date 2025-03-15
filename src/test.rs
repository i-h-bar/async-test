use crate::stats::Stats;
use futures::lock::Mutex;
use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use pyo3::{PyResult, Python};
use std::ops::{DerefMut};
use std::path::PathBuf;
use std::sync::Arc;

pub async fn run_test(path: PathBuf, stats: Arc<Mutex<Stats>>) -> PyResult<()> {
    let test = Python::with_gil(|py| {
        let coroutine = if let Some(name) = path.to_str() {
            let normalised: String = if let Some(stripped) = name.strip_prefix("./") {
                if let Some(stripped) = stripped.strip_suffix(".py") {
                    stripped.replace("/", ".").replace("\\", ".")
                } else {
                    stripped.replace("/", ".").replace("\\", ".")
                }
            } else {
                name.replace("/", ".").replace("\\", ".")
            };

            py.import(normalised)?
        } else {
            return Err(PyException::new_err("path is not valid UTF-8"));
        };

        pyo3_async_runtimes::tokio::into_future(coroutine.call_method0("test_case")?)
    })?;

    match test.await {
        Ok(_) => stats.lock().await.deref_mut().passed(),
        Err(_) => stats.lock().await.deref_mut().failed(),
    }

    Ok(())
}
