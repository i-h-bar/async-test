pub enum Outcome {
    PASSED,
    FAILED,
    ERRORED,
}

pub struct TestResult {
    pub name: String,
    pub outcome: Outcome,
    pub message: String,
    pub tb: String,
}
