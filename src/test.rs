use crate::results::{Outcome, TestResult};
use pyo3::exceptions::PyAssertionError;
use pyo3::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub struct Test {
    pub name: String,
    pub module_name: String,
    test: Pin<Box<dyn Future<Output = PyResult<PyObject>> + Send>>,
}

impl Test {
    pub fn from(
        name: String,
        module_name: String,
        test: Pin<Box<dyn Future<Output = PyResult<PyObject>> + Send>>,
    ) -> Self {
        Self {
            name,
            module_name,
            test,
        }
    }

    pub async fn run(self) -> TestResult {
        match self.test.await {
            Ok(_) => TestResult {
                name: self.name,
                outcome: Outcome::PASSED,
                message: None,
                tb: None,
            },
            Err(error) => Python::with_gil(|py| {
                if error.is_instance_of::<PyAssertionError>(py) {
                    TestResult {
                        name: self.name,
                        outcome: Outcome::FAILED,
                        message: Some(error.to_string()),
                        tb: extract_tb(&error, py),
                    }
                } else {
                    TestResult {
                        name: self.name,
                        outcome: Outcome::ERRORED,
                        message: Some(error.to_string()),
                        tb: extract_tb(&error, py),
                    }
                }
            }),
        }
    }
}

pub fn extract_tb(error: &PyErr, py: Python) -> Option<String> {
    error.traceback(py)?.format().ok()
}
