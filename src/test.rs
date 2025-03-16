use crate::results::{Outcome, TestResult};
use crate::stats::Stats;
use futures::lock::Mutex;
use pyo3::exceptions::{PyAssertionError, PyException};
use pyo3::prelude::*;
use pyo3::{PyResult, Python};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Arc;

fn modularise(path: PathBuf) -> PyResult<String> {
    if let Some(name) = path.to_str() {
        if let Some(stripped) = name.strip_prefix("./") {
            if let Some(stripped) = stripped.strip_suffix(".py") {
                Ok(stripped.replace("/", ".").replace("\\", "."))
            } else {
                Ok(stripped.replace("/", ".").replace("\\", "."))
            }
        } else {
            Ok(name.replace("/", ".").replace("\\", "."))
        }
    } else {
        Err(PyException::new_err("path is not valid UTF-8"))
    }
}

fn extract_tb(error: &PyErr, py: Python) -> Option<String> {
    error.traceback(py)?.format().ok()
}

pub async fn run_test(path: PathBuf, stats: Arc<Mutex<Stats>>) -> PyResult<()> {
    let module_name = modularise(path)?;
    let name = module_name.split(".").last().unwrap().to_string();

    let test = Python::with_gil(|py| {
        let module = py.import(module_name)?;
        pyo3_async_runtimes::tokio::into_future(module.call_method0("test_case")?)
    })?;

    let result = match test.await {
        Ok(_) => TestResult {
            name,
            outcome: Outcome::PASSED,
            message: None,
            tb: None,
        },
        Err(error) => Python::with_gil(|py| {
            if error.is_instance_of::<PyAssertionError>(py) {
                TestResult {
                    name,
                    outcome: Outcome::FAILED,
                    message: Some(error.to_string()),
                    tb: extract_tb(&error, py),
                }
            } else {
                TestResult {
                    name,
                    outcome: Outcome::ERRORED,
                    message: Some(error.to_string()),
                    tb: extract_tb(&error, py),
                }
            }
        }),
    };

    stats.lock().await.deref_mut().update(result);

    Ok(())
}
