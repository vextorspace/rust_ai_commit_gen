pub trait Ai {
    fn generate_commit_message(diff: String) -> String;
}