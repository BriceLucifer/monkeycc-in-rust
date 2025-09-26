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

    // 提前读取
    pub fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            let mut ch: u8 = 0;
            if let Some(x) = self.input.as_bytes().get(self.read_position) {
                ch = *x
            }
            return ch;
        }
    }

    pub fn next_token(&mut self) -> Token {
        // 默认初始化 EOF
        let mut token = Token::new_with_char(TokenType::Eof, '\0');

        self.skip_whitespace();

        match self.ch as char {
            '=' => {
                if self.peek_char() == '=' as u8 {
                    let ch = self.ch;
                    self.read_char();
                    token = Token::new_with_string(
                        TokenType::Eq,
                        format!("{}{}", ch as char, self.ch as char),
                    );
                } else {
                    token = Token::new_with_char(TokenType::Assign, self.ch as char);
                }
            }
            ';' => token = Token::new_with_char(TokenType::Semicolon, self.ch as char),
            '(' => token = Token::new_with_char(TokenType::Lparen, self.ch as char),
            ')' => token = Token::new_with_char(TokenType::Rparen, self.ch as char),
            '{' => token = Token::new_with_char(TokenType::Lbrace, self.ch as char),
            '}' => token = Token::new_with_char(TokenType::Rbrace, self.ch as char),
            ',' => token = Token::new_with_char(TokenType::Comma, self.ch as char),
            '+' => token = Token::new_with_char(TokenType::Plus, self.ch as char),
            '-' => token = Token::new_with_char(TokenType::Minus, self.ch as char),
            '/' => token = Token::new_with_char(TokenType::Slash, self.ch as char),
            '*' => token = Token::new_with_char(TokenType::Asterisk, self.ch as char),
            '<' => {
                if self.peek_char() == '=' as u8 {
                    let ch = self.ch;
                    self.read_char();
                    token = Token::new_with_string(
                        TokenType::Le,
                        format!("{}{}", ch as char, self.ch as char),
                    )
                } else {
                    token = Token::new_with_char(TokenType::Lt, self.ch as char)
                }
            }
            '>' => {
                if self.peek_char() == '=' as u8 {
                    let ch = self.ch;
                    self.read_char();
                    token = Token::new_with_string(
                        TokenType::Ge,
                        format!("{}{}", ch as char, self.ch as char),
                    )
                } else {
                    token = Token::new_with_char(TokenType::Gt, self.ch as char)
                }
            }
            '!' => {
                if self.peek_char() == '=' as u8 {
                    let ch = self.ch;
                    self.read_char();
                    token = Token::new_with_string(
                        TokenType::NotEq,
                        format!("{}{}", ch as char, self.ch as char),
                    )
                } else {
                    token = Token::new_with_char(TokenType::Bang, self.ch as char)
                }
            }
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
                    token = Token::new_with_char(TokenType::Illegal, self.ch as char)
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

// 判断是不是字母
pub fn is_letter(ch: u8) -> bool {
    return 'a' <= ch as char && ch as char <= 'z'
        || 'A' <= ch as char && ch as char <= 'Z'
        || ch as char == '_';
}

// 判断是不是数字
pub fn is_digital(ch: u8) -> bool {
    return '0' as u8 <= ch && ch <= '9' as u8;
}

// 为lexer 实现迭代器
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok.token_type == TokenType::Eof {
            None
        } else {
            Some(tok)
        }
    }
}
