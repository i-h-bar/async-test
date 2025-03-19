use uuid::Uuid;

pub enum Outcome {
    PASSED,
    FAILED,
    ERRORED,
}

pub struct TestResult<'a> {
    pub name: Option<&'a str>,
    pub module_name: &'a str,
    pub test_id: &'a Uuid,
    pub outcome: Outcome,
    pub message: Option<String>,
    pub tb: Option<String>,
}
