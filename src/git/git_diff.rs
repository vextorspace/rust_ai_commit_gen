use std::io::Error;
use std::ops::Deref;
use std::path::Path;
use std::process::Command;

trait DiffProvider {
    fn diff(path: Box<Path>) -> Result<String, Error>;
}

struct GitDiff{
}


impl GitDiff {
    pub fn new() -> Self {
        Self {}
    }
}

impl DiffProvider for GitDiff {
    fn diff(path: Box<Path>) -> Result<String, Error> {
        if let Some(path_str) = path.to_str() {
            let output = Command::new("git")
                .arg("diff")
                .arg(path_str)
                .output()?;

            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    String::from_utf8_lossy(&output.stderr).into_owned(),
                ));
            }
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid path provided.",
            ));
        }
    }
}