use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::{fs, thread};
use std::path::{Path, PathBuf};
use futures::future;

pub struct Tests {
    tests: Vec<String>,
}

impl Tests {
    pub fn find(dir: String) -> Self {
        todo!()
    }

    pub fn tests(&self) -> &Vec<String> {
        &self.tests
    }
}

pub async fn async_search(path: PathBuf) -> Vec<PathBuf> {
    let mut tests = Vec::new();

    if let Ok(mut path) = tokio::fs::read_dir(path).await {
        let mut sub_dirs = Vec::new();
        while let Ok(Some(entry)) = path.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_dir() {
                    let path = entry.path();
                    sub_dirs.push(async_search(path));
                } else if file_type.is_file() {
                    tests.push(entry.path())
                }
            }
        }

        for test in future::join_all(sub_dirs).await.into_iter().flatten() {
            tests.push(test);
        }
    }

    tests
}
