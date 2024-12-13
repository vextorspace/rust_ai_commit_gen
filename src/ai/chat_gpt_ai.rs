use dotenv::dotenv;
use std::env;
use super::ai_provider::AiProvider;
use tokio;
use llm_chain::{executor, options, parameters, prompt};
use anyhow::{anyhow, Result};
use llm_chain_openai::chatgpt::Executor;

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

    fn query_ai_for_commit_message(&self, diff: String, key: String) -> Result<String> {
        let prompt = self.make_diff_prompt(diff);

        let options = options! {
                ApiKey: key
            };

        let exec = executor!(chatgpt, options);

        let res = exec.map(|exec| {
            return Self::do_query_synchronously(prompt, &exec);
        });

        match res {
            Ok(result) => {Ok(result?)}
            Err(err) => {Err(anyhow!("failed to execute AI query: {}", err))}
        }
    }

    fn do_query_synchronously(prompt: String, exec: &Executor) -> Result<String> {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let res = runtime.block_on(async {
            prompt!(prompt)
                .run(&parameters!(), exec)
                .await
        });
        match res {
            Ok(result) => { Ok(result.to_string()) }
            Err(err) => { Err(anyhow!("failed to run executor query: {}", err)) }
        }
    }
}

impl AiProvider for ChatGptAi {
    fn generate_commit_message(&self, diff: String) -> Result<String> {

        if let Some(key) = self.api_key.clone() {
            self.query_ai_for_commit_message(diff, key)
        } else {
            Err(anyhow!("No API key."))
        }
    }
}

#[cfg(test)]
mod tests {
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
        let prompt = chat_gpt.make_diff_prompt(diff.clone());

        assert!(prompt.contains(diff.as_str()));
    }

    #[test]
    fn gets_a_real_commit_message() {
        let mut chat_gpt = ChatGptAi::new();
        chat_gpt.load_env();

        let diff = String::from("diff --git a/main.rs b/main.rs\nnew file mode 100644\nindex 0000000..e69de29\n--- a/main.rs\n+++ b/main.rs\n@@ -0,0 +1 @@\n+println!(\"Hippo\");\n");
        let commit_message = chat_gpt.generate_commit_message(diff.clone());

        assert!(commit_message.is_ok());
        let message = commit_message.unwrap();

        assert!(
            message.clone().contains("Hippo"),
            "commit message should contain Hippo");
    }
}