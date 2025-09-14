use crate::{
    ast::{Program, Statement},
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
                Some(stmt) => program.statements.push(stmt),
                None => self.next_token(),
            }
        }
        return Some(program);
    }

    pub fn parse_statement(&self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    pub fn parse_let_statement(&self) {}
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::{Ident, Statement};

    use super::*;

    fn test_let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let footbar = 838383;
        "#;

        let l = Lexer::new(input);
        let p = Parser::new(l);

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
                for stmt in p.statements {
                    for tt in tests.iter() {
                        // statement 中的name 和 tests ident中的Ident 看看对不对
                        assert!(test_let_statement(stmt.clone(), tt.0.clone()))
                    }
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
