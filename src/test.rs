use crate::results::{Outcome, TestResult};
use async_std::task::sleep;
use futures::FutureExt;
use futures::{pin_mut, select};
use pyo3::exceptions::PyAssertionError;
use pyo3::prelude::*;
use pyo3_async_runtimes;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::time::Duration;
use uuid::Uuid;

pub struct Test {
    pub id: Uuid,
    pub name: String,
    pub module_name: String,
    test: Option<Pin<Box<dyn Future<Output = PyResult<PyObject>> + Send>>>,
}

impl Test {
    pub fn from(
        name: String,
        module_name: String,
        test: Pin<Box<dyn Future<Output = PyResult<PyObject>> + Send>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            module_name,
            test: Some(test),
        }
    }

    pub fn failed_load(module_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            module_name,
            name: String::new(),
            test: None,
        }
    }

    pub async fn run(&mut self) -> TestResult {
        let test = self.test.take().unwrap().fuse();
        let timer = sleep(Duration::from_secs(5)).fuse();
        pin_mut!(timer, test);

        select! {
            outcome = test => {
                drop(test);
                match outcome{
            Ok(_) => TestResult {
                name: Some(&self.name),
                module_name: &self.module_name,
                test_id: &self.id,
                outcome: Outcome::PASSED,
                message: None,
                tb: None,
            },
            Err(error) => Python::with_gil(|py| {
                if error.is_instance_of::<PyAssertionError>(py) {
                    TestResult {
                        name: Some(&self.name),
                        module_name: &self.module_name,
                        test_id: &self.id,
                        outcome: Outcome::FAILED,
                        message: Some(error.to_string()),
                        tb: extract_tb(&error, py),
                    }
                } else {
                    TestResult {
                        name: Some(&self.name),
                        module_name: &self.module_name,
                        test_id: &self.id,
                        outcome: Outcome::ERRORED,
                        message: Some(error.to_string()),
                        tb: extract_tb(&error, py),
                    }
                }
            }),
        }
            },
            _ = timer => {
                    drop(test);
                                TestResult {
                test_id: &self.id,
                name: Some(&self.name),
                module_name: &self.module_name,
                outcome: Outcome::TIMEOUT,
                message: Some("Timeout after 5 seconds".to_string()),
                tb: None
            }
            }
        }


    }
}

pub fn extract_tb(error: &PyErr, py: Python) -> Option<String> {
    error.traceback(py)?.format().ok()
}
