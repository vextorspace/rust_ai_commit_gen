use std::io::Error;
use std::path::Path;
use std::process::Command;
use crate::git::diff_provider::DiffProvider;

pub struct GitDiff{
}

impl GitDiff {
    pub fn new() -> Self {
        Self {}
    }
}

impl DiffProvider for GitDiff {
    fn diff(&self, path: Box<Path>) -> Result<String, Error> {
        if let Some(path_str) = path.to_str() {
            let output = Command::new("git")
                .arg("diff")
                .arg(path_str)
                .output()?;

            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(Error::new(
                    std::io::ErrorKind::Other,
                    String::from_utf8_lossy(&output.stderr).into_owned(),
                ))
            }
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid path provided.",
            ))
        }
    }
}