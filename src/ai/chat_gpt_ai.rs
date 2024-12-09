pub struct ChatGptAi {
    api_key: Option<String>,
}

impl ChatGptAi {
    pub fn new() -> Self {
        Self {
            api_key: None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiates() {
        let ai = ChatGptAi::new();
        assert!(ai.api_key.is_none())

    }
}