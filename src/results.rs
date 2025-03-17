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


pub fn cli_format(result: &TestResult) -> (&str, &str) {
    match result.outcome {
        Outcome::PASSED => ("\u{2705}", "\x1b[1;32m"),
        Outcome::ERRORED => ("\u{1F6A8}", "\x1b[1;31m"),
        _ => ("\u{274c}", "\x1b[1;31m"),
    }
}