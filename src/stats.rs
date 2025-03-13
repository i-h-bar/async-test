pub struct Stats {
    total: usize,
    passed: usize,
    failed: usize,
    errored: usize,
    skipped: usize,
    timeout: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total: 0,
            passed: 0,
            failed: 0,
            errored: 0,
            skipped: 0,
            timeout: 0,
        }
    }
}
