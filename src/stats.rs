use crate::results::{Outcome, TestResult};

#[derive(Debug)]
pub struct Stats {
    total: usize,
    passed: Vec<String>,
    failed: Vec<(String, String, String)>,
    errored: Vec<(String, String, String)>,
    skipped: Vec<(String, String, String)>,
    timeout: Vec<(String, String, String)>,
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
            Outcome::FAILED => self.failed.push((result.name, result.message, result.tb)),
            Outcome::ERRORED => self.failed.push((result.name, result.message, result.tb)),
        }
    }
}
