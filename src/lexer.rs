use std::char;

use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: u8,
}

impl Lexer {
    // new lexer
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    // 逐渐读取
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            if let Some(x) = self.input.as_bytes().get(self.read_position) {
                self.ch = x.clone();
            }
        }
        self.position = self.read_position;
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

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        if let Some(x) = self.input.as_bytes().get(position..self.position) {
            return std::str::from_utf8(x).unwrap().to_string();
        } else {
            return String::new();
        }
    }
}

pub fn is_letter(ch: u8) -> bool {
    return 'a' <= ch as char && ch as char <= 'z'
        || 'A' <= ch as char && ch as char <= 'Z'
        || ch as char == '_';
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r"let five = 5;
            let ten = 10;
            let add = fn(x,y) {
                x + y;
            };
            let result = add(five, ten);
            ";

        let tests = vec![
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Function,
                literal: "fn".to_string(),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "y".to_string(),
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
                token_type: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "result".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
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

        // 创建Lexer
        let mut l: Lexer = Lexer::new(input);

        // 循环递归
        for tt in tests {
            let tok = l.next_token();
            // 对比
            // 首先两个type 要实现PartialEq trait
            assert_eq!(tt, tok);
        }
    }
}
