use crate::errors::{ErrorKind, SyncError};
use crate::file_list::FileListElement;
use bincode::{deserialize_from, serialize_into};
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufWriter};

pub fn read_cache(cache_path: &str) -> Result<HashMap<String, FileListElement>, SyncError> {
    let cache: HashMap<String, FileListElement>;
    cache = match fs::File::open(cache_path) {
        Ok(existing) => {
            let f = BufReader::new(existing);
            deserialize_from(f).map_err(|_| SyncError::new(ErrorKind::CacheError))?
        }
        Err(_) => HashMap::new(),
    };

    Ok(cache)
}

pub fn write_cache(
    cache_path: &str,
    results: Vec<FileListElement>,
) -> Result<HashMap<String, FileListElement>, SyncError> {
    let mut cache: HashMap<String, FileListElement> = HashMap::new();

    results.into_iter().for_each(|r| {
        cache.insert(r.cache_key().to_string(), r);
    });
    let mut f = BufWriter::new(fs::File::create(cache_path).unwrap());
    serialize_into(&mut f, &cache).map_err(|_| SyncError::new(ErrorKind::CacheError))?;
    Ok(cache)
}
