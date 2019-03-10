use std::error::Error;
use std::fmt;
use std::io::Error as ioError;
use std::path::StripPrefixError;

#[derive(Debug)]
pub struct SyncError(Box<ErrorKind>);

#[derive(Debug)]
pub enum ErrorKind {
    Prefix(StripPrefixError),
    IO(ioError),
}

impl SyncError {
    pub fn new(kind: ErrorKind) -> Self {
        SyncError(Box::new(kind))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for SyncError {
    fn description(&self) -> &str {
        match *self.kind() {
            ErrorKind::IO(ref err) => err.description(),
            ErrorKind::Prefix(ref err) => err.description(),
        }
    }
}

impl From<ioError> for SyncError {
    fn from(err: ioError) -> Self {
        SyncError::new(ErrorKind::IO(err))
    }
}
