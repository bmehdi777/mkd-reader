use crate::ast::lexer::*;
use crate::ast::token::*;

struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Parser {
        let current_token: Token = l.next_token();
        let peek_token: Token = l.next_token();
        Parser { l, current_token, peek_token, errors: vec![]}
    }
    pub fn errors(&self) -> Vec<String> {
        self.errors.to_owned()
    }
    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
}


