use std::io::Error;
use std::path::Path;
use crate::ai::ai_provider::Ai;
use super::diff_provider::DiffProvider;

pub struct CommitGenerator {
    differ: Option<Box<dyn DiffProvider>>,
    ai: Option<Box<dyn Ai>>
}

impl CommitGenerator {
    pub fn new() -> Self {
        Self {
            ai: None,
            differ: None,
        }
    }

    pub fn with_ai(mut self, ai: Box<dyn Ai>) -> Self {
        self.ai = Some(ai);
        self
    }

    pub fn with_differ(mut self, diff_provider: Box<dyn DiffProvider>) -> Self {
        self.differ = Some(diff_provider);
        self
    }

    pub fn generate_commit_message(&self, path: &Path) -> Result<String, Error> {
        if let Some(differ) = &self.differ {
            let diff = differ.diff(path)?;
            Ok(diff)
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "No differ provided.",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ai::chat_gpt_ai::ChatGptAi;
    use crate::git::git_diff::GitDiff;
    use super::*;

    #[test]
    fn instantiates() {
        let differ = GitDiff::new();
        let ai = ChatGptAi::new();
        let _commit_generator = CommitGenerator::new()
            .with_differ(Box::new(differ))
            .with_ai(Box::new(ai));
    }
}