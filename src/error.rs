use syn_inline_mod::{Error as InlineError, InlineError as InlineResultError};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

pub struct ParseError {
    pub location: Location,
    pub message: String,
    pub caller: Option<Location>,
}

pub struct IoError {
    pub file: PathBuf,
    pub message: String,
    pub caller: Option<Location>,
}

pub enum Error {
    Io(IoError),
    Parse(ParseError),
}

pub struct MultiError(pub Vec<Error>);

impl Error {
    pub fn from_io_error(file: &Path, caller: Option<Location>, error: &std::io::Error) -> Error {
        Error::Io(IoError{
            file: file.to_owned(),
            message: error.to_string(),
            caller: caller,
        })
    }

    pub fn from_parse_error(file: &Path, caller: Option<Location>, error: &syn::Error) -> Error {
        let lc = error.span().start();
        Error::Parse(ParseError{
            location: Location {
                file: file.to_owned(),
                line: lc.line,
                column: lc.column,
            },
            message: error.to_string(),
            caller: caller,
        })
    }

    pub fn from_inline_error(error: &InlineResultError) -> Error {
        let lc = error.src_span().start();
        let caller = Some(Location{
            file: error.src_path().to_owned(),
            line: lc.line,
            column: lc.column,
        });
        match error.kind() {
            InlineError::Io(err) => Error::from_io_error(error.path(), caller, err),
            InlineError::Parse(err) => Error::from_parse_error(error.path(), caller, err),
        }
    }
}

impl MultiError {
    pub fn from_io_error(file: &Path, error: &std::io::Error) -> MultiError {
        MultiError(vec![Error::from_io_error(file, None, error)])
    }

    pub fn from_parse_error(file: &Path, error: &syn::Error) -> MultiError {
        MultiError(vec![Error::from_parse_error(file, None, error)])
    }

    pub fn from_inline_errors(errors: &[InlineResultError]) -> MultiError {
        MultiError(errors.iter().map(|error| Error::from_inline_error(error)).collect())
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "required from {}:{}:{}", self.file.display(), self.line, self.column + 1)
    }
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "io error while reading {}:\n  {}", self.file.display(), self.message)?;
        if let Some(location) = &self.caller {
            write!(f, "\nrequired from {}", location)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error at {}:\n  {}", self.location, self.message)?;
        if let Some(location) = &self.caller {
            write!(f, "\nrequired from {}", location)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(error) => std::fmt::Display::fmt(error, f),
            Error::Parse(error) => std::fmt::Display::fmt(error, f),
        }
    }   
}

impl std::fmt::Display for MultiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in self.0.iter() {
            write!(f, "{}\n\n", error)?
        }
        match self.0.len() {
            0 => {},
            1 => write!(f, "reported 1 error")?,
            n => write!(f, "reported {} errors", n)?,
        }
        Ok(())
    }   
}
