#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EOF,
    ILLEGAL,

    TITLE,
    PARAGRAPH,

    BOLD,
    ITALIC,
    STRIKETHROUGH,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token { token_type, value }
    }

    pub fn lookup(identifier: &str) -> TokenType {
        match identifier {
            "#" => TokenType::TITLE,
            _ => TokenType::PARAGRAPH,
        }
    }
}
