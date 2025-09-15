use crate::{
    ast::{Expr, Ident, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    l: Lexer,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        // 初始化
        let mut p: Parser = Parser {
            l: lexer,
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        p.next_token();
        p.next_token();

        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != TokenType::Eof {
            let stmt = self.parse_statement();
            match stmt {
                Statement::None => {}
                _ => {
                    program.statements.push(stmt);
                }
            }
            self.next_token();
        }
        return Some(program);
    }

    // 解析statement
    pub fn parse_statement(&mut self) -> Statement {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement().unwrap(),
            _ => Statement::None,
        }
    }

    // return 一个Option<Statement> => Statement::Let{name: Ident, value: Expr}
    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        // 因为我提前预判到cur_token 是TokenType::Let
        // 直接就可以peek 是不是ident
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        // 开始创建Let Statement
        let stmt = Statement::Let {
            name: Ident(self.cur_token.literal.clone()),
            // skip value expression
            value: Expr::Default,
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: we are skipping the value handle
        // encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(stmt);
    }

    // 辅助函数 查看当前tokentype 是否匹配
    pub fn cur_token_is(&self, token_type: TokenType) -> bool {
        return self.cur_token.token_type == token_type;
    }

    // 辅助函数 查看下一个tokentype 是否匹配
    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        return self.peek_token.token_type == token_type;
    }

    // 如果接下来的类型是和参数token_type 匹配 滚动下一个next token 然后返回true
    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::{Ident, Statement};

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let footbar = 838383;
        "#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        match program {
            Some(p) => {
                // 判断是不是三个let stmt
                if p.statements.len() != 3 {
                    eprintln!(
                        "program.statements does not contain 3 statements. got {}",
                        p.statements.len()
                    );
                }
                // 三个Ident x, y, foobar
                let tests: Vec<Ident> = vec![
                    Ident("x".to_string()),
                    Ident("y".to_string()),
                    Ident("footbar".to_string()),
                ];
                for (stmt, tt) in p.statements.iter().zip(tests.iter()) {
                    assert!(test_let_statement(stmt.clone(), tt.0.clone()))
                }
            }
            None => {
                eprintln!("parse_program() returned None");
                return;
            }
        }
    }

    pub fn test_let_statement(stmt: Statement, tt: String) -> bool {
        match stmt {
            Statement::Let { name, value } => {
                if name.0 != tt {
                    return false;
                }
                return true;
            }
            Statement::None => {
                eprintln!("It is not a let Statement");
                return false;
            }
        }
    }
}
