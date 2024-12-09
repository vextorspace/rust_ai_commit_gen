pub trait Ai {
    fn generate_commit_message(&self, diff: String) -> String;
}