use std::path::PathBuf;

/// An error that can occur in this crate.
#[derive(Debug)]
pub struct NomosError {
    errorKind: ErrorKind,
}

impl NomosError {
    pub fn new(kind: ErrorKind) -> NomosError {
        NomosError { errorKind: kind }
    }
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.errorKind
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    DirectoryNotEmpty { path: PathBuf },
    IoOpen { source: std::io::Error, path: PathBuf },
    IoWrite { source: std::io::Error, path: PathBuf },
    CreateDirectory { source: std::io::Error, path: PathBuf },
}
