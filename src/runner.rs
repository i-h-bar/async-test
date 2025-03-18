use crate::progress::Bars;
use crate::results::{Outcome, TestResult};
use crate::stats::Stats;
use crate::test::{extract_tb, Test};
use futures::lock::{Mutex, MutexGuard};
use pyo3::{PyErr, Python};
use std::ops::DerefMut;
use std::sync::Arc;

pub struct SuiteRunner {
    stats: Arc<Mutex<Stats>>,
    bars: Mutex<Bars>,
}

impl SuiteRunner {
    pub fn new() -> Self {
        let stats = Arc::new(Mutex::new(Stats::new()));
        let bars = Bars::new();

        Self {
            stats,
            bars: Mutex::new(bars),
        }
    }

    pub async fn run_test(&self, mut test: Test) {
        self.stats.lock().await.deref_mut().total += 1;

        self.bars.lock().await.register(
            test.id.clone(),
            format!("{}: {}", test.module_name, test.name.clone()),
        );

        let result = test.run().await;

        self.bars.lock().await.finish(&result);

        self.stats.lock().await.deref_mut().update(result);
    }

    pub async fn load_failed(&self, test: Test, error: PyErr) {
        self.bars
            .lock()
            .await
            .register(test.id.clone(), test.module_name.clone());

        let result = Python::with_gil(|py| TestResult {
            name: None,
            module_name: &test.module_name,
            test_id: &test.id,
            outcome: Outcome::ERRORED,
            message: Some(error.to_string()),
            tb: extract_tb(&error, py),
        });

        self.bars.lock().await.finish(&result);

        self.stats.lock().await.deref_mut().update(result);
    }

    pub async fn stats(&self) -> MutexGuard<Stats> {
        self.stats.lock().await
    }
}
