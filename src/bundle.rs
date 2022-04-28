use syn_inline_mod::{Error as InlineError, InlinerBuilder};
use std::path::Path;
use crate::error::MultiError;

pub fn bundle_file(src: &Path) -> Result<String, MultiError> {
    match InlinerBuilder::new().parse_and_inline_modules(src) {
        Ok(f) if f.has_errors() == false => Ok(prettyplease::unparse(&f.into_output_and_errors().0)),
        Ok(f) => Err(MultiError::from_inline_errors(f.errors())),
        Err(InlineError::Io(err)) => Err(MultiError::from_io_error(src, &err)),
        Err(InlineError::Parse(err)) => Err(MultiError::from_parse_error(src, &err)),
    }
}
