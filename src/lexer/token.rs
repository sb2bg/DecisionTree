use crate::lexer::token_type::TokenType;

pub struct Token {
    value: Option<String>,
    token_type: TokenType,
}

impl Token {
    pub fn new(value: Option<String>, token_type: TokenType) -> Self {
        Self { value, token_type }
    }

    pub fn get_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let to_str: &str = self.token_type.into();
        format!("{}{}", to_str, if let Some(value) = &self.value { format!("({})", value) } else { String::new() })
    }
}