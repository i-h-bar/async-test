
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

    pub fn passed(&mut self) {
        self.passed += 1;
    }

    pub fn failed(&mut self) {
        self.failed += 1;
    }
}
