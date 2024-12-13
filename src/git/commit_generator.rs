use super::diff_provider::DiffProvider;
use crate::ai::ai_provider::AiProvider;
use anyhow::{anyhow, Result};
use std::path::Path;


pub struct CommitGenerator {
    differ: Option<Box<dyn DiffProvider>>,
    ai: Option<Box<dyn AiProvider>>
}

impl CommitGenerator {
    pub fn new() -> Self {
        Self {
            ai: None,
            differ: None,
        }
    }

    pub fn with_ai(mut self, ai: Box<dyn AiProvider>) -> Self {
        self.ai = Some(ai);
        self
    }

    pub fn with_differ(mut self, diff_provider: Box<dyn DiffProvider>) -> Self {
        self.differ = Some(diff_provider);
        self
    }

    pub fn generate_commit_message(&self, path: &Path) -> Result<String> {
        if let Some(differ) = &self.differ {
            let diff = differ.diff(path)?;

            if let Some(ai) = &self.ai {
                ai.generate_commit_message(diff)
            } else {
                Err(anyhow!("No AI."))
            }
        } else {
            Err(anyhow!("No diff provider."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::chat_gpt_ai::ChatGptAi;
    use crate::git::git_diff::GitDiff;
    use mockall::{mock, Sequence};

    #[test]
    fn instantiates() {
        let differ = GitDiff::new();
        let ai = ChatGptAi::new();
        let _commit_generator = CommitGenerator::new()
            .with_differ(Box::new(differ))
            .with_ai(Box::new(ai));
    }

    #[test]
    fn generates_commit_message() {
        mock! {
            pub DiffProviderMock {}
            impl DiffProvider for DiffProviderMock {
                fn diff(&self, path: &Path) -> Result<String>;
            }
        }

        mock! {
            pub AiProviderMock {}
            impl AiProvider for AiProviderMock {
                fn generate_commit_message(&self, diff: String) -> Result<String>;
            }
        }
        let mut seg = Sequence::new();

        let mut differ = MockDiffProviderMock::new();
        differ.expect_diff()
            .times(1)
            .in_sequence(&mut seg)
            .returning(|_| Ok(String::from("diffy hippo")));


        let mut ai = MockAiProviderMock::new();
        ai.expect_generate_commit_message()
            .times(1)
            .in_sequence(&mut seg)
            .returning(|_| Ok(String::from("Hippo")));

        let commit_generator = CommitGenerator::new()
            .with_differ(Box::new(differ))
            .with_ai(Box::new(ai));

        let path = Path::new("src/git/git_diff.rs");
        let _message = commit_generator.generate_commit_message(path);
    }
}