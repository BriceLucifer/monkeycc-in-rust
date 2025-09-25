#[cfg(test)]
mod parser_test {
    use monkeycc::ast::{Expr, Ident, Statement};
    use monkeycc::lexer::Lexer;
    use monkeycc::parser::Parser;

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

        // 检查parse 有没有errors
        check_parser_errors(&p);

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

                // 压缩结构体 依次匹配
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

    // 辅助测试let statement
    pub fn test_let_statement(stmt: Statement, tt: String) -> bool {
        match stmt {
            Statement::Let { name, .. } => {
                if name.0 != tt {
                    return false;
                }
                return true;
            }
            _ => return false,
        }
    }

    // test return statement function
    #[test]
    pub fn test_return_statements() {
        let input = r#"
           return 5;
           return 10;
           return 993322;
        "#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        match program {
            Some(p) => {
                if p.statements.len() != 3 {
                    eprintln!(
                        "program.statements does not contain 3 statements, got = {}",
                        p.statements.len()
                    );
                }
                for stmt in p.statements {
                    match stmt {
                        Statement::Return(value) => {
                            // assert_eq!("5".to_string(), value.string())
                        }
                        _ => {
                            eprintln!("stmt not return statement, got = {:?}", stmt);
                        }
                    }
                }
            }
            None => {
                eprintln!("Error parser_program return None");
                return;
            }
        }
    }

    // just for test ident
    #[test]
    fn test_identifier_expression() {
        let input = "footbar;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        match program {
            Some(p) => {
                if p.statements.len() != 1 {
                    eprintln!(
                        "program has not enough statements. got = {}",
                        p.statements.len()
                    );
                }
                let stmt = p.statements[0].clone();
                match stmt {
                    Statement::Expression(e) => {
                        assert_eq!("footbar".to_string(), e.expression.string())
                    }
                    _ => {
                        eprintln!("Not a Expression statement")
                    }
                }
            }
            None => eprintln!("parse_program() returns None"),
        }
    }

    // test for integer value
    #[test]
    pub fn test_integer_expression() {
        let input = "5;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        // 检查解析器是否有errors
        check_parser_errors(&p);

        match program {
            Some(p) => {
                if p.statements.len() != 1 {
                    panic!(
                        "program has not enough statements. got = {}",
                        p.statements.len()
                    );
                }

                let stmt = p.statements[0].clone();
                match stmt {
                    Statement::Expression(expr) => {
                        if let Expr::Integer(i) = expr.expression {
                            assert_eq!(5, i);
                        }
                    }
                    _ => {
                        panic!("Not Expression statement");
                    }
                }
            }
            None => {
                panic!("parse_program() error");
            }
        }
    }

    // test expression
    #[test]
    pub fn test_prefix_expression() {
        struct Tprefix {
            // 输入
            pub input: String,
            // 前置测试
            pub prefix: Expr,
        }
        let prefix_test: Vec<Tprefix> = vec![
            Tprefix {
                input: "!5".to_string(),
                prefix: Expr::Prefix {
                    op: monkeycc::token::TokenType::Bang,
                    right: Box::new(Expr::Integer(5)),
                },
            },
            Tprefix {
                input: "-15".to_string(),
                prefix: Expr::Prefix {
                    op: monkeycc::token::TokenType::Minus,
                    right: Box::new(Expr::Integer(15)),
                },
            },
        ];

        for t in prefix_test {
            let l = Lexer::new(&t.input);
            let mut p = Parser::new(l);

            let programs = p.parse_program();
            match programs {
                Some(program) => {
                    check_parser_errors(&p);

                    if program.statements.len() != 1 {
                        eprintln!(
                            "program.statements does not contain 1 statements, got {} instead",
                            program.statements.len()
                        );
                    }

                    let stmt = program.statements[0].clone();
                    match stmt {
                        Statement::Expression(expr_stmt) => {
                            assert_eq!(expr_stmt.expression.string(), t.prefix.string());
                        }
                        other => {
                            eprintln!(
                                "program.statements[0] is not a expression, got {} instead",
                                other.string()
                            )
                        }
                    }
                }
                None => {
                    eprintln!("Error parse_program()");
                    return;
                }
            }
        }
    }

    // 辅助函数检查是否需要check_parser_errors()
    pub fn check_parser_errors(p: &Parser) {
        let errors = p.errors();

        if errors.len() == 0 {
            return;
        }

        eprintln!("Parser has {} errors", errors.len());
        for msg in errors {
            eprintln!("- parser error: {}", msg)
        }

        panic!("Fail the check parser errors")
    }
}
