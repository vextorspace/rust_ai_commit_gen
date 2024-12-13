use super::ai_provider::AiProvider;
use anyhow::{anyhow, Context, Error, Result};
use llm_chain::traits::ExecutorCreationError;
use llm_chain_openai::chatgpt::Executor;
    fn query_ai_for_commit_message(&self, diff: String, key: String) -> Result<String> {
        let prompt = self.make_diff_prompt(diff);
        let options = options! {
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