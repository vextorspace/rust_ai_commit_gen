use anyhow::{Result, anyhow};
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
    fn diff(&self, path: &Path) -> Result<String> {
        let output = Command::new("git")
            .arg("diff")
            .arg("HEAD")
            .current_dir(path)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(anyhow!(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ))
        }
    }
}