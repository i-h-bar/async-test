use crate::results::{Outcome, TestResult};
use std::fmt::Display;

pub struct Stats {
    pub total: usize,
    pub passed: Vec<String>,
    pub failed: Vec<(String, String, String)>,
    pub errored: Vec<(String, String, String)>,
    pub skipped: Vec<(String, String, String)>,
    pub timeout: Vec<(String, String, String)>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total: 0,
            passed: Vec::new(),
            failed: Vec::new(),
            errored: Vec::new(),
            skipped: Vec::new(),
            timeout: Vec::new(),
        }
    }

    pub fn update(&mut self, result: TestResult) {
        match result.outcome {
            Outcome::PASSED => self.passed.push(match result.name {
                Some(name) => name.to_string(),
                None => result.module_name.to_string(),
            }),
            Outcome::FAILED => self.failed.push((
                match result.name {
                    Some(name) => name.to_string(),
                    None => result.module_name.to_string(),
                },
                result
                    .message
                    .unwrap_or_else(|| "Failed to get error".to_string()),
                result.tb.unwrap_or_else(|| "Failed to get tb".to_string()),
            )),
            Outcome::ERRORED => self.errored.push((
                match result.name {
                    Some(name) => name.to_string(),
                    None => result.module_name.to_string(),
                },
                result
                    .message
                    .unwrap_or_else(|| "Failed to get error".to_string()),
                result.tb.unwrap_or_else(|| "Failed to get tb".to_string()),
            )),
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n\nTotal    - {}\n\nPassed   - {}\nFailed   - {}\nErrored  - {}\nTimedout - {}\nSkipped  - {}",
            self.total,
            self.passed.len(),
            self.failed.len(),
            self.errored.len(),
            self.timeout.len(),
            self.skipped.len()
        )?;
        Ok(())
    }
}
