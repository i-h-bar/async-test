use crate::runner::SuiteRunner;
use crate::test::Test;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::{PyResult, Python};
use std::path::PathBuf;

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

pub async fn run_module(module: String, suite: &SuiteRunner) -> PyResult<()> {
    let name = module.split(".").last().unwrap().to_string();
    let tests: PyResult<Vec<Test>> = Python::with_gil(|py| {
        let module_name = module.clone();
        let module = py.import(module)?;
        Ok(module
            .getattr("__dict__")?
            .try_iter()?
            .into_iter()
            .filter_map(|item| {
                let item = item.ok()?.extract::<String>().ok()?;
                if item.starts_with("test") {
                    Some(Test::from(
                        item.clone(),
                        module_name.clone(),
                        Box::pin(
                            pyo3_async_runtimes::tokio::into_future(
                                module.call_method0(&item).ok()?,
                            )
                            .ok()?,
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect())
    });

    match tests {
        Ok(tests) => {
            futures::future::join_all(tests.into_iter().map(|test| suite.run_test(test))).await;
            ()
        },
        Err(error) => suite.load_failed(name, error).await,
    }

    Ok(())
}
