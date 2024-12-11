use std::io::Error;
use std::path::Path;

pub trait DiffProvider {
    fn diff(&self, path: &Path) -> Result<String, Error>;
}
