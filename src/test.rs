use crate::results::{Outcome, TestResult};
use crate::stats::Stats;
use futures::lock::Mutex;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use pyo3::exceptions::{PyAssertionError, PyException};
use pyo3::prelude::*;
use pyo3::{PyResult, Python};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub fn modularise(path: PathBuf) -> PyResult<String> {
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

pub async fn run_test(
    module: String,
    stats: Arc<Mutex<Stats>>,
    multi_bar: &MultiProgress,
    longest_name: usize,
) -> PyResult<()> {
    let name = module.split(".").last().unwrap().to_string();
    let name_clone = name.clone();

    let bar = multi_bar.add(ProgressBar::new_spinner());
    bar.set_message(name_clone);
    bar.enable_steady_tick(Duration::from_millis(100));

    let test = Python::with_gil(|py| {
        let module = py.import(module)?;
        pyo3_async_runtimes::tokio::into_future(module.call_method0("test_case")?)
    });

    let result = match test {
        Ok(test) => match test.await {
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
        },
        Err(error) => Python::with_gil(|py| TestResult {
            name,
            outcome: Outcome::ERRORED,
            message: Some(error.to_string()),
            tb: extract_tb(&error, py),
        }),
    };

    let (indicator, colour) = match result.outcome {
        Outcome::PASSED => ("\u{2705}", "\x1b[1;32m"),
        Outcome::ERRORED => ("\u{1F6A8}", "\x1b[1;31m"),
        _ => ("\u{274c}", "\x1b[1;31m"),
    };

    let reason = match &result.message {
        Some(reason) => reason,
        None => "",
    };

    let padding_size = longest_name - result.name.len();
    let padding = (0..padding_size).map(|_| " ").collect::<String>();

    bar.set_message(format!(
        "{}{}{} - {}   {}\x1b[0m",
        colour, &result.name, padding, indicator, reason
    ));
    bar.finish();
    stats.lock().await.deref_mut().update(result);

    Ok(())
}
