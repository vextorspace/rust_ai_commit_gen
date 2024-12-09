use crate::git::git_diff::*;
use crate::git::diff_provider::DiffProvider;

pub struct CommitGenerator {
    differ: Option<Box<dyn DiffProvider>>,
}

impl CommitGenerator {
    pub fn new() -> Self {
        Self {
            differ: None,
        }
    }

    pub fn with_differ(&self, diff_provider: Box<dyn DiffProvider>) -> Self {
        Self {
            differ: Some(diff_provider),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiates() {
        let differ = GitDiff::new();
        let commit_generator = CommitGenerator::new()
            .with_differ(Box::new(differ));
    }
}