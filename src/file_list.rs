use crate::errors::{ErrorKind, SyncError};
use crypto::digest::Digest;
use crypto::md5::Md5;
use serde_derive::*;
use std::time::SystemTime;

use std::fs;
use std::io::Read;

use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileListElement {
    pub path: PathBuf,
    pub checksum: String,
    pub size: u64,
    pub modified: SystemTime,
    pub permissions_mode: u32,
    cache_prefix: String,
}

impl<'a> FileListElement {
    pub fn new(path: PathBuf, prefix: &str) -> Self {
        FileListElement {
            checksum: String::new(),
            size: 0,
            modified: SystemTime::UNIX_EPOCH,
            permissions_mode: 0,
            cache_prefix: path
                .strip_prefix(prefix)
                .map_err(|e| SyncError::new(ErrorKind::Prefix(e)))
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),

            path: path,
        }
    }

    pub fn calc_md5(&mut self) {
        let mut file =
            fs::File::open(&self.path).expect(&format!("failed to open file {:?}", self.path));
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .expect(&format!("failed to read file {:?}", self.path));

        let mut hasher = Md5::new();
        hasher.input(&buf);
        self.checksum = hasher.result_str();
    }

    pub fn cache_key(&self) -> &str {
        &self.cache_prefix
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
