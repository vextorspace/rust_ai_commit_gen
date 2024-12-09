use dotenv::dotenv;
use std::env;

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
}