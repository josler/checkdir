mod cache;
mod errors;
mod file_list;

use crate::cache::*;
use crate::errors::{ErrorKind, SyncError};
use crate::file_list::FileListElement;
use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> Result<(), SyncError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = PathBuf::from(&args[0]);
    let cache_prefix = &args[0];
    let mut results = Vec::new();
    let canonical = path.canonicalize()?;
    let cache_dir = PathBuf::from("/tmp/checkdir_cache");
    fs::create_dir_all(&cache_dir).unwrap();
    let cache_path = cache_dir.join(canonical.file_name().unwrap());

    let mut cache: HashMap<String, FileListElement> = read_cache(&cache_path.to_str().unwrap())?;

    visit_stack(&mut cache, path, cache_prefix, &mut results)?;

    // parallel sort all the file paths
    results.par_sort_by(|a, b| a.path.cmp(&b.path));

    // parallel calculate md5 sums for all files
    results = results
        .into_par_iter()
        .map(|mut v| {
            if !cache.contains_key(v.cache_key()) {
                v.calc_md5();
            }
            v
        })
        .collect();

    // write all the file paths and md5s into a single string buffer. We can't
    // parallelise this as we're mutating a single resource
    let mut all_file_md5s = String::new();
    results.iter().for_each(|v| {
        let path = v
            .path_without_prefix(cache_prefix)
            .expect("failed to get path without prefix");
        all_file_md5s.push_str(&format!("{:} {:}\n", v.checksum, path));
    });

    // println!("{:}", all_file_md5s);

    // Calculate the final md5 hash from the file list
    let mut hasher = Md5::new();
    hasher.input_str(&all_file_md5s);

    println!("{:}", hasher.result_str());

    write_cache(&cache_path.to_str().unwrap(), results)?;
    Ok(())
}

fn visit_stack(
    cache: &mut HashMap<String, FileListElement>,
    path: PathBuf,
    prefix: &str,
    results: &mut Vec<FileListElement>,
) -> Result<(), SyncError> {
    // not following symlinks
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !should_skip(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let mut element = FileListElement::new(entry.path().to_path_buf(), prefix);
            let metadata = entry.metadata().unwrap();

            // If our cache has this path, and it's still the same size and modified time of the cache,
            // then we can reuse it
            if cache.contains_key(element.cache_key()) {
                let found = &cache[element.cache_key()];
                if found.size == metadata.len()
                    && found.modified == metadata.modified().unwrap()
                    && found.permissions_mode == metadata.permissions().mode()
                {
                    // element = found.clone();
                    element.size = found.size;
                    element.modified = found.modified;
                    element.permissions_mode = found.permissions_mode;
                    element.checksum = found.checksum.clone();
                    results.push(element);
                    continue;
                } else {
                    // mismatch of metadata or modified
                    cache.remove(element.cache_key());
                }
            }

            // otherwise we should regenerate it
            element.size = metadata.len();
            element.modified = metadata.modified().unwrap();
            element.permissions_mode = metadata.permissions().mode();
            results.push(element);
        }
    }
    Ok(())
}

// check if we should skip top level entries
fn should_skip(entry: &walkdir::DirEntry) -> bool {
    let filename = entry.file_name().to_str();

    // top level directory excludes
    if entry.depth() == 1 && entry.file_type().is_dir() {
        return dir_is_name(".git", filename)
            || dir_is_name("tmp", filename)
            || dir_is_name("log", filename)
            || dir_is_name(".idea", filename)
            || dir_is_name("avatars", filename);
    }

    // specific file excludes
    path_ends_with("spec/examples.txt", entry.path())
}

// exact name of directory
fn dir_is_name(dir: &str, filename: Option<&str>) -> bool {
    match filename {
        Some(name) => name == dir,
        None => false,
    }
}

fn path_ends_with(dir: &str, path: &Path) -> bool {
    path.ends_with(dir)
}
