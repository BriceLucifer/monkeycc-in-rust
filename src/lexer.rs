use std::char;

use crate::token::{Token, TokenType, lookup_ident};

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
                self.ch = *x;
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        // 默认初始化 EOF
        let mut token = Token::new(TokenType::Eof, '\0');

        self.skip_whitespace();

        match self.ch as char {
            '=' => token = Token::new(TokenType::Assign, self.ch as char),
            ';' => token = Token::new(TokenType::Semicolon, self.ch as char),
            '(' => token = Token::new(TokenType::Lparen, self.ch as char),
            ')' => token = Token::new(TokenType::Rparen, self.ch as char),
            '{' => token = Token::new(TokenType::Lbrace, self.ch as char),
            '}' => token = Token::new(TokenType::Rbrace, self.ch as char),
            ',' => token = Token::new(TokenType::Comma, self.ch as char),
            '+' => token = Token::new(TokenType::Plus, self.ch as char),
            '-' => token = Token::new(TokenType::Minus, self.ch as char),
            '/' => token = Token::new(TokenType::Slash, self.ch as char),
            '*' => token = Token::new(TokenType::Asterisk, self.ch as char),
            '<' => token = Token::new(TokenType::Lt, self.ch as char),
            '>' => token = Token::new(TokenType::Gt, self.ch as char),
            '!' => token = Token::new(TokenType::Bang, self.ch as char),
            '\0' => {
                token.literal = "".to_string();
                token.token_type = TokenType::Eof;
            }
            _ => {
                if is_letter(self.ch) {
                    token.literal = self.read_identifier();
                    token.token_type = lookup_ident(token.literal.clone());
                    return token;
                } else if is_digital(self.ch) {
                    token.token_type = TokenType::Int;
                    token.literal = self.read_number();
                    return token;
                } else {
                    token = Token::new(TokenType::Illegal, self.ch as char)
                }
            }
        }
        self.read_char();
        return token;
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        return self.input.get(position..self.position).unwrap().to_string();
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digital(self.ch) {
            self.read_char();
        }
        return self.input.get(position..self.position).unwrap().to_string();
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch as char == ' '
            || self.ch as char == '\t'
            || self.ch as char == '\n'
            || self.ch as char == '\r'
        {
            self.read_char();
        }
    }
}

pub fn is_letter(ch: u8) -> bool {
    return 'a' <= ch as char && ch as char <= 'z'
        || 'A' <= ch as char && ch as char <= 'Z'
        || ch as char == '_';
}

pub fn is_digital(ch: u8) -> bool {
    return '0' as u8 <= ch && ch <= '9' as u8;
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
            !-/*5;
            5 < 10 > 5;
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
            Token {
                token_type: TokenType::Bang,
                literal: "!".to_string(),
            },
            Token {
                token_type: TokenType::Minus,
                literal: "-".to_string(),
            },
            Token {
                token_type: TokenType::Slash,
                literal: "/".to_string(),
            },
            Token {
                token_type: TokenType::Asterisk,
                literal: "*".to_string(),
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
                token_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::Lt,
                literal: "<".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::Gt,
                literal: ">".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
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
