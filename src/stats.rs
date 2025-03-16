use crate::results::{Outcome, TestResult};

#[derive(Debug)]
pub struct Stats {
    pub total: usize,
    pub passed: Vec<String>,
    pub failed: Vec<(String, String, String)>,
    pub errored: Vec<(String, String, String)>,
    pub skipped: Vec<(String, String, String)>,
    pub timeout: Vec<(String, String, String)>,
}

impl Stats {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            passed: Vec::with_capacity(total),
            failed: Vec::with_capacity(total),
            errored: Vec::with_capacity(total),
            skipped: Vec::with_capacity(total),
            timeout: Vec::with_capacity(total),
        }
    }

    pub fn update(&mut self, result: TestResult) {
        match result.outcome {
            Outcome::PASSED => self.passed.push(result.name),
            Outcome::FAILED => self.failed.push((
                result.name,
                result.message.unwrap_or_else(|| "Failed to get error".to_string()),
                result.tb.unwrap_or_else(|| "Failed to get tb".to_string()),
            )),
            Outcome::ERRORED => self.errored.push((
                result.name,
                result.message.unwrap_or_else(|| "Failed to get error".to_string()),
                result.tb.unwrap_or_else(|| "Failed to get tb".to_string()),
            )),
        }
    }
}
