use crate::results::{cli_format, Outcome, TestResult};
use crate::stats::Stats;
use crate::test::{extract_tb, Test};
use futures::lock::{Mutex, MutexGuard};
use indicatif::{MultiProgress, ProgressBar};
use pyo3::{PyErr, Python};
use std::ops::DerefMut;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct SuiteRunner {
    stats: Arc<Mutex<Stats>>,
    longest_name: AtomicUsize,
    progress_bar: MultiProgress,
}

impl SuiteRunner {
    pub fn new() -> Self {
        let stats = Arc::new(Mutex::new(Stats::new()));
        let progress_bar = MultiProgress::new();

        Self {
            stats,
            longest_name: AtomicUsize::new(0),
            progress_bar,
        }
    }

    pub async fn run_test(&self, test: Test) {
        self.stats.lock().await.deref_mut().total += 1;

        let bar = self.progress_bar.add(ProgressBar::new_spinner());
        let name = format!("{}: {}", test.module_name, test.name.clone());
        bar.set_message(name.clone());
        bar.enable_steady_tick(Duration::from_millis(100));
        if name.len() > self.longest_name.load(Ordering::Relaxed) {
            self.longest_name.swap(name.len(), Ordering::Relaxed);
        }

        let result = test.run().await;

        let (indicator, colour) = cli_format(&result);
        let reason = match &result.message {
            Some(reason) => reason,
            None => "",
        };

        let padding_size = self.longest_name.load(Ordering::Relaxed) - name.len();
        let padding = (0..padding_size).map(|_| " ").collect::<String>();

        bar.set_message(format!(
            "{}{}{} - {}   {}\x1b[0m",
            colour, &name, padding, indicator, reason
        ));
        bar.finish();
        self.stats.lock().await.deref_mut().update(result);
    }

    pub async fn load_failed(&self, name: String, error: PyErr) {
        let bar = self.progress_bar.add(ProgressBar::new_spinner());
        bar.set_message(name.clone());
        bar.enable_steady_tick(Duration::from_millis(100));
        if name.len() > self.longest_name.load(Ordering::Relaxed) {
            self.longest_name.swap(name.len(), Ordering::Relaxed);
        }

        let result = Python::with_gil(|py| TestResult {
            name,
            outcome: Outcome::ERRORED,
            message: Some(error.to_string()),
            tb: extract_tb(&error, py),
        });

        let (indicator, colour) = cli_format(&result);
        let reason = match &result.message {
            Some(reason) => reason,
            None => "",
        };

        let padding_size = self.longest_name.load(Ordering::Relaxed) - result.name.len();
        let padding = (0..padding_size).map(|_| " ").collect::<String>();

        bar.set_message(format!(
            "{}{}{} - {}   {}\x1b[0m",
            colour, &result.name, padding, indicator, reason
        ));
        bar.finish();
        self.stats.lock().await.deref_mut().update(result);
    }

    pub async fn stats(&self) -> MutexGuard<Stats> {
        self.stats.lock().await
    }
}
