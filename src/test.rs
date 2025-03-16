use crate::results::{Outcome, TestResult};
use crate::stats::Stats;
use futures::lock::Mutex;
use pyo3::exceptions::{PyAssertionError, PyException};
use pyo3::prelude::*;
use pyo3::{PyResult, Python};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use indicatif::{MultiProgress, ProgressBar};

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

pub async fn run_test(path: PathBuf, stats: Arc<Mutex<Stats>>, multi_bar: &MultiProgress) -> PyResult<()> {
    let module_name = modularise(path)?;
    let name = module_name.split(".").last().unwrap().to_string();

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let bar = multi_bar.add(ProgressBar::new_spinner());
    bar.set_message(format!("{} Running...", &name));

    let handle = tokio::spawn(async move {
        loop {
            bar.tick();

            if running_clone.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(100)).await;
            } else {
                break
            }
        }

        bar
    });

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

    running.swap(false, Ordering::Relaxed);
    let bar = handle.await.unwrap();

    bar.set_message(format!("{} finished", &result.name));
    bar.finish();
    stats.lock().await.deref_mut().update(result);

    Ok(())
}
