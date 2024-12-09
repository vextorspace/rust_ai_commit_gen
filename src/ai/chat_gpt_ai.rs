use dotenv::dotenv;
use std::env;
use super::ai::Ai;

pub struct ChatGptAi {
    api_key: Option<String>,
}

impl ChatGptAi {
    pub fn new() -> Self {
        Self {
            api_key: None,
        }
    }

    pub fn load_env(&mut self) {
        dotenv().ok();
        self.api_key = env::var("OPENAI_API_KEY").ok();
    }

    pub(crate) fn make_diff_prompt(&self, diff: String) -> String {
        let prompt = format!("
                            You are a terse and efficient developer.
                            You only state the most important changes in commit messages.
                            Each change should be on its own line.
                            Each change message should be 50 characters or less.
                            Try to keep each change message below 6 words if possible.
                            An added or removed file should be mentioned in the message.
                                the diff is: {diff}:

                            Write a non-generic commit message. ", diff = diff);

        prompt
    }
}

impl Ai for ChatGptAi {
    fn generate_commit_message(&self, diff: String) -> String {
        todo!();
    }
}
#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::*;

    #[test]
    fn instantiates() {
        let mut ai = ChatGptAi::new();
        assert!(ai.api_key.is_none());
        ai.load_env();
        assert!(ai.api_key.is_some());
    }

    #[test]
    fn generates_prompt_from_diff() {
        let mut chat_gpt = ChatGptAi::new();
        chat_gpt.load_env();
        
        let diff = String::from("diff --git a/main.rs b/main.rs\nnew file mode 100644\nindex 0000000..e69de29\n--- a/main.rs\n+++ b/main.rs\n@@ -0,0 +1 @@\n+println!(\"Hippo\");\n");
        let prompt = chat_gpt.make_diff_prompt(diff);

        assert!(prompt.contains("Hippo"));
    }
}