use crate::errors::{ErrorKind, SyncError};
use fxhash::FxHasher;
use std::hash::Hasher;

use std::fs;
use std::io::Read;

use std::path::PathBuf;

#[derive(Debug)]
pub struct FileListElement {
    pub path: PathBuf,
    pub checksum: String,
}

impl<'a> FileListElement {
    pub fn new(path: PathBuf) -> Self {
        FileListElement {
            path: path,
            checksum: String::new(),
        }
    }

    pub fn calculate_checksum(&mut self) {
        let mut file = fs::File::open(&self.path).expect("failed to open file");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("failed to read file");

        let mut hasher = FxHasher::default();
        hasher.write(&buf);
        self.checksum = hasher.finish().to_string();
    }

    pub fn path_without_prefix(&self, prefix: &str) -> Result<&str, SyncError> {
        Ok(self
            .path
            .strip_prefix(prefix)
            .map_err(|e| SyncError::new(ErrorKind::Prefix(e)))?
            .to_str()
            .unwrap())
    }
}
