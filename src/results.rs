pub enum Outcome {
    PASSED,
    FAILED,
    ERRORED,
}

pub struct TestResult {
    pub name: String,
    pub outcome: Outcome,
    pub message: Option<String>,
    pub tb: Option<String>,
}
