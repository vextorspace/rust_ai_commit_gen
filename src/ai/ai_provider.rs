use anyhow::Result;

pub trait AiProvider {
    fn generate_commit_message(&self, diff: String) -> Result<String>;
}