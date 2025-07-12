use crate::results::{Outcome, TestResult};
use std::fmt::Display;
use crate::progress::cli_colours;

pub struct Stats {
    pub total: usize,
    pub passed: Vec<String>,
    pub failed: Vec<(String, String, String)>,
    pub errored: Vec<(String, String, String)>,
    pub skipped: Vec<(String, String)>,
    pub timeout: Vec<(String, String)>,
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
            Outcome::TIMEOUT => self.timeout.push((
                result.name.unwrap_or_else(|| "foo").to_string(),
                result
                    .message
                    .unwrap_or_else(|| "Failed to get error".to_string()),
            )),
            Outcome::SKIPPED => self.skipped.push((
                result.name.unwrap_or_else(|| "foo").to_string(),
                result
                    .message
                    .unwrap_or_else(|| "Failed to get error".to_string()),
            ))
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (p_emoji, p_colour) = cli_colours(&Outcome::PASSED);
        let (f_emoji, f_colour) = cli_colours(&Outcome::FAILED);
        let (e_emoji, e_colour) = cli_colours(&Outcome::ERRORED);
        let (t_emoji, t_colour) = cli_colours(&Outcome::TIMEOUT);
        let (s_emoji, s_colour) = cli_colours(&Outcome::SKIPPED);
        
        write!(
            f,
            "\n\nTotal    - {}\n\n {} {}Passed   - {}\x1b[0m\n {} {}Failed   - {}\x1b[0m\n {} {}Errored  - {}\x1b[0m\n {} {}Timedout - {}\x1b[0m\n {} {}Skipped  - {}\x1b[0m",
            self.total,
            p_emoji,
            p_colour,
            self.passed.len(),
            f_emoji,
            f_colour,
            self.failed.len(),
            e_emoji,
            e_colour,
            self.errored.len(),
            t_emoji,
            t_colour,
            self.timeout.len(),
            s_emoji,
            s_colour,
            self.skipped.len(),
        )?;
        Ok(())
    }
}
