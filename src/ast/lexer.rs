use crate::ast::token::*;
use crate::ast::utils::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    character: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            character: 0,
        };
        l.read_character();
        l
    }

    fn read_character(&mut self) {
        self.character = if self.read_position >= self.input.len() {
            0
        } else {
            self.input.bytes().nth(self.read_position).unwrap()
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_words(&mut self) -> String {
        let first_pos: usize = self.position;
        while is_letter(self.character) {
            self.read_character();
        }
        self.input[first_pos..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitespace();

        match self.character {
            b'#' => {
                if self.peek_character() == b'#' {
                    println!("here");
                    let mut title = String::from("#");
                    while self.peek_character() == b'#' {
                        self.read_character();
                        title.push('#');
                    }
                    println!("{:}", title);
                    tok = Token::new(TokenType::TITLE, title);
                } else {
                    tok = Token::new(TokenType::TITLE, String::from("#"));
                }
            }
            b'*' | b'_' | b'~' => {
                if self.peek_character() == self.character {
                    tok = Token::new(
                        TokenType::BOLD,
                        String::from(format!("{}{}", (self.character as char), (self.peek_character() as char))),
                    );
                } else {
                    tok = Token::new(
                        TokenType::PARAGRAPH,
                        String::from(format!("{}", (self.character as char))),
                    );
                }
            }
            0 => tok = Token::new(TokenType::EOF, String::new()),
            _ => {
                if !(is_letter(self.character) || is_digit(self.character)) {
                    return Token::new(TokenType::ILLEGAL, (self.character as char).to_string());
                }
                let words: String = self.read_words();
                tok = Token::new(TokenType::PARAGRAPH, words);
            }
        }
        self.read_character();
        tok
    }

    fn peek_character(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.bytes().nth(self.read_position).unwrap()
        }
    }

    fn skip_whitespace(&mut self) {
        if self.character == b' ' || self.character == b'\n' || self.character == b'\t' {
            self.read_character();
        }
    }
}
