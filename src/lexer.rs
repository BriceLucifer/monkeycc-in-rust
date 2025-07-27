use std::char;

use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub postion: usize,
    pub read_position: usize,
    pub ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            postion: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            if let Some(x) = self.input.as_bytes().get(self.read_position) {
                self.ch = x.clone();
            }
        }
        self.postion = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        // 转换char
        let ch = self.ch as char;
        // 默认初始化
        let mut token = Token::new(TokenType::Eof, ch);

        match ch {
            '=' => token = Token::new(TokenType::Assign, ch),
            ';' => token = Token::new(TokenType::Semicolon, ch),
            '(' => token = Token::new(TokenType::Lparen, ch),
            ')' => token = Token::new(TokenType::Rparen, ch),
            '{' => token = Token::new(TokenType::Lbrace, ch),
            '}' => token = Token::new(TokenType::Rbrace, ch),
            ',' => token = Token::new(TokenType::Comma, ch),
            '+' => token = Token::new(TokenType::Plus, ch),
            '\0' => {
                token.literal = "".to_string();
                token.token_type = TokenType::Eof;
            }
            _ => {}
        }
        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "=+(){},;";

        let tests = vec![
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        ];

        let mut l: Lexer = Lexer::new(input);

        for tt in tests {
            let tok = l.next_token();

            assert_eq!(tt, tok);
        }
    }
}
