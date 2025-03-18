use crate::results::{Outcome, TestResult};
use indicatif::{MultiProgress, ProgressBar};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

struct Bar {
    spinner: ProgressBar,
    message: String,
    length: usize,
    failure_reason: Option<String>,
    indicator: Option<&'static str>,
    colour: Option<&'static str>,
    padding: String,
}

pub struct Bars {
    bar_generator: MultiProgress,
    bars: HashMap<Uuid, Bar>,
    longest: usize,
}

impl Bar {
    fn update(&mut self, result: &TestResult, longest: usize) {
        let (indicator, colour) = cli_format(&result);
        self.indicator = Some(indicator);
        self.colour = Some(colour);

        self.failure_reason = match &result.message {
            Some(reason) => Some(reason.to_owned()),
            None => None,
        };

        let reason = match self.failure_reason.as_ref() {
            Some(failure_reason) => failure_reason.to_string(),
            None => "".to_string(),
        };

        self.padding = (0..longest - self.message.len()).map(|_| " ").collect::<String>();

        self.spinner.set_message(format!(
            "{}{}{} - {}   {}\x1b[0m",
            colour, self.message, self.padding, indicator, reason
        ));
        self.spinner.finish();
    }

    fn update_padding(&mut self, new_length: usize) {
        self.padding = (0..new_length).map(|_| " ").collect::<String>();
        let reason = match self.failure_reason.as_ref() {
            Some(failure_reason) => failure_reason.to_string(),
            None => "".to_string(),
        };

        let indicator = self.indicator.unwrap_or_else(|| "");

        self.spinner.set_message(format!(
            "{}{}{} - {}   {}\x1b[0m",
            self.colour.expect("Colour"), self.message, self.padding, indicator, reason
        ));
        self.spinner.tick();
    }
}

impl Bars {
    pub fn new() -> Self {
        Self {
            bar_generator: MultiProgress::new(),
            bars: HashMap::new(),
            longest: 0
        }
    }

    pub fn register(&mut self, uuid: Uuid, message: String) {
        let length = message.len();

        if length > self.longest {
            self.longest = length;
        }

        let spinner = self.bar_generator.add(ProgressBar::new_spinner());
        spinner.set_message(message.clone());
        spinner.enable_steady_tick(Duration::from_millis(100));
        let bar = Bar {
            spinner,
            message,
            length,
            failure_reason: None,
            indicator: None,
            colour: None,
            padding: String::new(),
        };

        self.bars.insert(uuid, bar);
    }

    pub fn finish(&mut self, result: &TestResult) {
        let Some(bar) = self.bars.get_mut(&result.test_id) else {
            return;
        };

        bar.update(&result, self.longest);
    }

    fn check_padding(&mut self) {
        for (_, bar) in self.bars.iter_mut() {
            let new_length = self.longest - bar.message.len();
            if bar.padding.len() > new_length {
                bar.update_padding(new_length);
            }
        }
    }
}

pub fn cli_format(result: &TestResult) -> (&'static str, &'static str) {
    match result.outcome {
        Outcome::PASSED => ("\u{2705}", "\x1b[1;32m"),
        Outcome::ERRORED => ("\u{1F6A8}", "\x1b[1;31m"),
        _ => ("\u{274c}", "\x1b[1;31m"),
    }
}
