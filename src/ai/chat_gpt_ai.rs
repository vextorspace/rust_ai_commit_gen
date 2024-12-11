use tokio;
use llm_chain::{executor, options, parameters, prompt};
    fn generate_commit_message(&self, diff: String) -> Result<String, Box<dyn std::error::Error>> {

        if let Some(key) = self.api_key.clone() {
            let prompt = self.make_diff_prompt(diff);

            let options = options! {
                ApiKey: key
            };

            let exec = executor!(chatgpt, options);
            match exec {
                Ok(exec) => {

                    let runtime = tokio::runtime::Runtime::new()?;
                    runtime.block_on(async {
                        let res = prompt!(prompt)
                            .run(&parameters!(), &exec) // ...and run it
                            .await;
                        res.map_or(Err("No response from AI".into()), |r| Ok(r.to_string()))
                    })
                }
                Err(err) => Err(err.into()),
            }
        } else { 
            Err("API key is not set in the environment variables".into())
        }
        let prompt = chat_gpt.make_diff_prompt(diff.clone());

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
        assert!(message.clone().len() < 60,
            "commit message should be less than 60 characters");
    }