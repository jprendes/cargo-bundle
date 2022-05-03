use syn_inline_mod::{Error as InlineError, InlinerBuilder};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use quote::ToTokens;
use crate::error::MultiError;

pub struct Bundle(syn::File);

impl Bundle {
    pub fn new(src: &Path) -> Result<Bundle, MultiError> {
        match InlinerBuilder::new().parse_and_inline_modules(src) {
            Ok(f) if f.has_errors() == false => Ok(Bundle(f.into_output_and_errors().0)),
            Ok(f) => Err(MultiError::from_inline_errors(f.errors())),
            Err(InlineError::Io(err)) => Err(MultiError::from_io_error(src, &err)),
            Err(InlineError::Parse(err)) => Err(MultiError::from_parse_error(src, &err)),
        }
    }

    pub fn minify(&self) -> String {
        let input = &self.0;
        input.into_token_stream().to_string()
    }

    pub fn prettify(&self) -> String {
        prettify_prettyplease(&self.0)
            .unwrap_or_else(|_| {
                let ugly = self.minify();
                prettify_rustfmt(&ugly).unwrap_or(ugly)
            })
    }
}

fn prettify_rustfmt(input: &str) -> anyhow::Result<String> {
    let mut child = Command::new("rustfmt")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(input.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        anyhow::bail!("{}", String::from_utf8(output.stderr)?)
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn prettify_prettyplease(input: &syn::File) -> std::thread::Result<String> {
    std::panic::catch_unwind(|| {
        prettyplease::unparse(input)
    })
}
