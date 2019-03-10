mod errors;
mod file_list;

use crate::errors::{ErrorKind, SyncError};
use crate::file_list::FileListElement;
use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<(), SyncError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = PathBuf::from(&args[0]);

    let mut results = Vec::new();

    visit_stack(path, &mut results)?;

    // parallel sort all the file paths
    results.par_sort_by(|a, b| a.path.cmp(&b.path));

    // parallel calculate md5 sums for all files
    results = results
        .into_par_iter()
        .map(|mut v| {
            v.calc_md5();
            v
        })
        .collect();

    // write all the file paths and md5s into a single string buffer. We can't
    // parallelise this as we're mutating a single resource
    let mut all_file_md5s = String::new();
    results.iter().for_each(|v| {
        let path = v
            .path_without_prefix(&args[0])
            .expect("failed to get path without prefix");
        all_file_md5s.push_str(&format!("{:} {:}\n", v.checksum, path));
    });

    // println!("{:}", all_file_md5s);

    // Calculate the final md5 hash from the file list
    let mut hasher = Md5::new();
    hasher.input_str(&all_file_md5s);

    println!("{:}", hasher.result_str());
    Ok(())
}

fn visit_stack(path: PathBuf, results: &mut Vec<FileListElement>) -> Result<(), SyncError> {
    // not following symlinks
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !should_skip(e))
        .filter_map(|e| e.ok())
    {
        if !entry.path().is_dir() {
            results.push(FileListElement::new(entry.path().to_path_buf()));
        }
    }
    Ok(())
}

fn should_skip(entry: &walkdir::DirEntry) -> bool {
    is_top_level(".git", entry) || is_top_level("tmp", entry) || is_top_level("vendor", entry)
}

fn is_top_level(dir: &str, entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(dir))
        .unwrap_or(false)
}
