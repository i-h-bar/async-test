use crate::results::{Outcome, TestResult};

#[derive(Debug)]
pub struct Stats {
    total: usize,
    passed: usize,
    failed: usize,
    errored: usize,
    skipped: usize,
    timeout: usize,
}

impl Stats {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            passed: 0,
            failed: 0,
            errored: 0,
            skipped: 0,
            timeout: 0,
        }
    }

    pub fn update(&mut self, result: TestResult) {
        match result.outcome {
            Outcome::PASSED => self.passed += 1,
            Outcome::FAILED => self.failed += 1,
            Outcome::ERRORED => self.errored += 1,
        }
    }
}
