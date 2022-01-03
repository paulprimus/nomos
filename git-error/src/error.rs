use std::path::PathBuf;
use std::string::FromUtf8Error;

/// An error that can occur in this crate.
#[derive(Debug)]
pub struct NomosError {
    error_kind: ErrorKind,
}

impl NomosError {
    pub fn new(kind: ErrorKind) -> NomosError {
        NomosError { error_kind: kind }
    }
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.error_kind
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    DirectoryNotEmpty { path: PathBuf },
    IoOpen { source: std::io::Error, path: PathBuf },
    IoWrite { source: std::io::Error, path: PathBuf },
    CreateDirectory { source: std::io::Error, path: PathBuf },
    FromUtf8Error {source: FromUtf8Error},
    InvalidByteSliceLength {len: usize},
    InvalidHexEncodingLength {len: usize},
}

impl From<FromUtf8Error> for NomosError{
    fn from(error: FromUtf8Error) -> Self {
        NomosError::new(ErrorKind::FromUtf8Error {source: error})
    }
}