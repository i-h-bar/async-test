use std::sync::Arc;
use std::{fs, thread};
use std::sync::mpsc::{channel, Sender};

pub struct Tests {
    tests: Vec<String>,
}

impl Tests {
    pub fn find(dir: String) -> Self {
        let (tx, rx) = channel();
        let mut tests = Vec::new();
        let tx = Arc::new(tx);
        thread::spawn(move || search_directory(dir, Arc::clone(&tx)));

        for test in rx {
            tests.push(test);
        }

        Self { tests }
    }

    pub fn tests(&self) -> &Vec<String> {
        &self.tests
    }
}

fn search_directory(path: String, tx: Arc<Sender<String>>) -> () {
    if let Ok(path) = fs::read_dir(path) {
        for entry in path {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let copy = Arc::clone(&tx);
                        thread::spawn(move || {
                            search_directory(
                                entry.path().to_str().unwrap().to_string(),
                                copy,
                            )
                        });
                    } else if file_type.is_file() {
                        tx.send(entry.path().to_str().unwrap().to_string()).unwrap();
                    }
                }
            }
        }
    }
}
