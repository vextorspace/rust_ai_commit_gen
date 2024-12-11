use tokio;

pub trait Ai {
    fn generate_commit_message(&self, diff: String) -> Result<String, Box<dyn std::error::Error>>;
}