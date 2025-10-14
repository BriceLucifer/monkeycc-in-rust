#[cfg(test)]
mod parser_test {
    use std::iter::zip;

    use monkeycc::ast::{Expr, Ident, Statement};
    use monkeycc::lexer::Lexer;
    use monkeycc::parser::Parser;
    use monkeycc::token::TokenType;

    #[test]
    fn test_let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let footbar = 838383;
        "#;

        struct IdentValue(&'static str, &'static str);

        let tests = vec![
            IdentValue("x", "5"),
            IdentValue("y", "10"),
            IdentValue("footbar", "838383"),
        ];

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        // 检查parse 有没有errors
        check_parser_errors(&p);

        match program {
            Some(p) => {
                // 判断是不是三个let stmt
                assert_eq!(
                    3,
                    p.statements.len(),
                    "let statements is expected {}, got {}",
                    3,
                    p.statements.len()
                );
                for (t, i_v) in p.statements.iter().zip(tests.iter()) {
                    match &t {
                        &Statement::Let { name, value } => {
                            assert_eq!(name.string(), i_v.0);
                            assert_eq!(value.string(), i_v.1);
                        }
                        _ => {
                            panic!("not a Let statement");
                        }
                    }
                }
            }
            None => {
                panic!("parse_program() returned None");
            }
        }
    }

    // test return statement function
    #[test]
    pub fn test_return_statements() {
        let input = r#"
           return 5;
           return 10;
           return 993322;
           return 1 + 2;
        "#;
        let values = vec!["5", "10", "993322", "(1 + 2)"];

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        match program {
            Some(p) => {
                if p.statements.len() != 4 {
                    eprintln!(
                        "program.statements does not contain 3 statements, got = {}",
                        p.statements.len()
                    );
                }
                for (stmt, val) in zip(p.statements, values) {
                    match stmt {
                        Statement::Return(value) => {
                            assert_eq!(&value.string(), val);
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

    // test ident
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
            // 前置操作符
            pub op: TokenType,
            // 操作数
            pub right: i64,
        }

        // 创建prefix_test 数组
        let prefix_test: Vec<Tprefix> = vec![
            Tprefix {
                input: "!5;".to_string(),
                op: monkeycc::token::TokenType::Bang,
                right: 5,
            },
            Tprefix {
                input: "-15;".to_string(),
                op: monkeycc::token::TokenType::Minus,
                right: 15,
            },
            Tprefix {
                input: "!true".to_string(),
                op: monkeycc::token::TokenType::Bang,
                right: true as i64,
            },
            Tprefix {
                input: "!false".to_string(),
                op: monkeycc::token::TokenType::Bang,
                right: false as i64,
            },
        ];

        // 循环判断
        for t in prefix_test {
            let l = Lexer::new(&t.input);
            let mut p = Parser::new(l);

            let programs = p.parse_program();
            match programs {
                Some(program) => {
                    check_parser_errors(&p);

                    // 断言
                    assert_eq!(
                        program.statements.len(),
                        1,
                        "want 1 stmt, got {}",
                        program.statements.len()
                    );

                    let stmt = program.statements[0].clone();
                    match stmt {
                        Statement::Expression(expr_stmt) => {
                            if let Expr::Prefix { op, right } = expr_stmt.expression {
                                assert_eq!(op, t.op);
                                // box 指针解引用
                                match *right {
                                    Expr::Boolean(bool) => {
                                        assert_eq!(
                                            bool as i64, t.right,
                                            "expect {}, got {}",
                                            t.right, bool as i64
                                        );
                                    }
                                    Expr::Integer(int) => {
                                        assert_eq!(int, t.right, "expect {}, got {}", t.right, int)
                                    }
                                    _ => {}
                                }
                            }
                        }
                        other => {
                            panic!(
                                "program.statements[0] is not a expression, got {} instead",
                                other.string()
                            )
                        }
                    }
                }
                None => {
                    panic!("Error parse_program()");
                }
            }
        }
    }

    #[test]
    pub fn test_infix_expression() {
        struct Tinfix {
            input: String,
            left: i64,
            op: TokenType,
            right: i64,
        }

        let infix_test: Vec<Tinfix> = vec![
            Tinfix {
                input: "5 + 5;".to_string(),
                left: 5,
                op: TokenType::Plus,
                right: 5,
            },
            Tinfix {
                input: "5 - 5;".to_string(),
                left: 5,
                op: TokenType::Minus,
                right: 5,
            },
            Tinfix {
                input: "5 * 5;".to_string(),
                left: 5,
                op: TokenType::Asterisk,
                right: 5,
            },
            Tinfix {
                input: "5 / 5;".to_string(),
                left: 5,
                op: TokenType::Slash,
                right: 5,
            },
            Tinfix {
                input: "5 > 5;".to_string(),
                left: 5,
                op: TokenType::Gt,
                right: 5,
            },
            Tinfix {
                input: "5 < 5;".to_string(),
                left: 5,
                op: TokenType::Lt,
                right: 5,
            },
            Tinfix {
                input: "5 == 5;".to_string(),
                left: 5,
                op: TokenType::Eq,
                right: 5,
            },
            Tinfix {
                input: "5 != 5;".to_string(),
                left: 5,
                op: TokenType::NotEq,
                right: 5,
            },
        ];

        for t in infix_test {
            let l = Lexer::new(&t.input);
            let mut p = Parser::new(l);

            let programs = p.parse_program();
            match programs {
                Some(program) => {
                    assert_eq!(
                        program.statements.len(),
                        1,
                        "want 1 stmt, got {}",
                        program.statements.len()
                    );

                    let stmt = program.statements[0].clone();
                    match stmt {
                        Statement::Expression(expre) => {
                            if let Expr::Infix { left, op, right } = expre.expression {
                                assert_eq!(op, t.op);
                                if let Expr::Integer(i) = *left {
                                    assert_eq!(i, t.left)
                                }
                                if let Expr::Integer(i) = *right {
                                    assert_eq!(i, t.right)
                                }
                            }
                        }
                        _ => panic!("program.statement[0] is not an expression statement"),
                    }
                }
                None => {
                    panic!("parser_program() error");
                }
            }
        }
    }

    #[test]
    pub fn test_operator_precedence_parsing() {
        struct Toperator {
            input: &'static str,
            expected: &'static str,
        }
        let tests = vec![
            Toperator {
                input: "-a * b",
                expected: "((-a) * b)",
            },
            Toperator {
                input: "!-a",
                expected: "(!(-a))",
            },
            Toperator {
                input: "a + b + c",
                expected: "((a + b) + c)",
            },
            Toperator {
                input: "a + b - c",
                expected: "((a + b) - c)",
            },
            Toperator {
                input: "a * b * c",
                expected: "((a * b) * c)",
            },
            Toperator {
                input: "a * b / c",
                expected: "((a * b) / c)",
            },
            Toperator {
                input: "a + b / c",
                expected: "(a + (b / c))",
            },
            Toperator {
                input: "a + b * c + d / e - f",
                expected: "(((a + (b * c)) + (d / e)) - f)",
            },
            Toperator {
                input: "3 + 4; -5 * 5",
                expected: "(3 + 4)((-5) * 5)",
            },
            Toperator {
                input: "5 > 4 == 3 < 4",
                expected: "((5 > 4) == (3 < 4))",
            },
            Toperator {
                input: "5 < 4 != 3 > 4",
                expected: "((5 < 4) != (3 > 4))",
            },
            Toperator {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            },
            Toperator {
                input: "3 + 4 * 5 != 3 * 1 + 4 * 5",
                expected: "((3 + (4 * 5)) != ((3 * 1) + (4 * 5)))",
            },
            Toperator {
                input: "true",
                expected: "true",
            },
            Toperator {
                input: "false",
                expected: "false",
            },
            Toperator {
                input: "3 > 5 == false",
                expected: "((3 > 5) == false)",
            },
            Toperator {
                input: "3 < 5 == true",
                expected: "((3 < 5) == true)",
            },
            Toperator {
                input: "a + add(b * c) + d",
                expected: "((a + add((b * c))) + d)",
            },
            Toperator {
                input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            },
            Toperator {
                input: "add(a+b+c *d /f +g)",
                expected: "add((((a + b) + ((c * d) / f)) + g))",
            },
        ];

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            match program {
                Some(p) => {
                    let actual = p.string();
                    assert_eq!(
                        &actual, tt.expected,
                        "expected {}, actual got {}",
                        tt.expected, &actual
                    )
                }
                None => panic!("Error parse_program()"),
            }
        }
    }

    // test boolean
    #[test]
    pub fn test_boolean() {
        let input = "true;false;";
        let tests = vec![true, false];

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let programs = parser.parse_program().unwrap();
        assert_eq!(2, programs.statements.len());

        for (stmt, tt) in zip(programs.statements, tests) {
            if let Statement::Expression(expr) = stmt {
                match expr.expression {
                    Expr::Boolean(b) => assert_eq!(b, tt),
                    _ => panic!(
                        "not a Expr::Boolean(bool), got {}",
                        expr.expression.string()
                    ),
                }
            }
        }
    }

    // test if expression
    #[test]
    pub fn test_if_expression() {
        let input = "if (x < y) { x }";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        check_parser_errors(&p);

        let program = p.parse_program().unwrap();
        assert_eq!(
            1,
            program.statements.len(),
            "program body does not contain {} statements. got {}",
            1,
            program.statements.len()
        );

        match program.statements[0].clone() {
            Statement::Expression(expr) => {
                if let Expr::IfExpression {
                    condition,
                    consequence,
                    alternative,
                } = expr.expression
                {
                    assert_eq!("(x < y)", &condition.string());
                    assert_eq!("x", &consequence.string());
                    assert_eq!("None", &alternative.string())
                }
            }
            _ => panic!("Not a Expression"),
        }
    }

    #[test]
    pub fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        check_parser_errors(&p);
        let program = p.parse_program().unwrap();

        assert_eq!(
            1,
            program.statements.len(),
            "expected 1 statements, got {}",
            program.statements.len()
        );

        match program.statements[0].clone() {
            Statement::Expression(expr_stmt) => match expr_stmt.expression {
                Expr::IfExpression {
                    condition,
                    consequence,
                    alternative,
                } => {
                    assert_eq!("(x < y)", &condition.string());
                    assert_eq!("x", &consequence.string());
                    assert_eq!("y", &alternative.string());
                }
                _ => {
                    panic!("Not a if expression")
                }
            },
            _ => {
                panic!("Not a Expression");
            }
        }
    }

    // test function literal
    #[test]
    pub fn test_fn_literal() {
        let input = "fn(x, y) {x + y;}";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        check_parser_errors(&p);

        assert_eq!(
            1,
            program.statements.len(),
            "expected 1 statements, got {}",
            program.statements.len()
        );
        match program.statements[0].clone() {
            Statement::Expression(expr_stmt) => {
                if let Expr::Fn(func) = expr_stmt.expression {
                    assert_eq!(
                        func.parameters.len(),
                        2,
                        "Expected 2 parameters, got {}",
                        func.parameters.len()
                    );
                    assert_eq!(func.parameters[0].string(), "x".to_string());
                    assert_eq!(func.parameters[1].string(), "y".to_string());
                    match *func.body {
                        Statement::Block(expr_body) => {
                            assert_eq!(expr_body.statements.len(), 1);
                            assert_eq!(expr_body.string(), "(x + y)".to_string());
                        }
                        _ => {
                            panic!("function body stmt is not Statement::Expression(expr_body)");
                        }
                    }
                }
            }
            _ => {
                panic!("program.statements[0] is not a Statement::Expression");
            }
        }
    }

    // 测试函数arguement
    #[test]
    pub fn test_fn_parameter_parsing() {
        pub struct TestP {
            pub input: &'static str,
            pub expected: Vec<&'static str>,
        }

        let tests = vec![
            TestP {
                input: "fn() {}",
                expected: Vec::new(),
            },
            TestP {
                input: "fn(x) {}",
                expected: vec!["x"],
            },
            TestP {
                input: "fn(x, y) {}",
                expected: vec!["x", "y"],
            },
        ];

        for t in tests {
            let l = Lexer::new(t.input);
            let mut p = Parser::new(l);
            check_parser_errors(&p);

            let program = p.parse_program().unwrap();

            // 确保是一句
            assert_eq!(
                1,
                program.statements.len(),
                "expected 1 statements, got {}",
                program.statements.len()
            );

            let stmt = program.statements[0].clone();

            match stmt {
                Statement::Expression(e) => {
                    if let Expr::Fn(func) = e.expression {
                        assert_eq!(func.parameters.len(), t.expected.len());
                        for (i, i_t) in zip(func.parameters, t.expected) {
                            assert_eq!(&i.string(), i_t)
                        }
                    }
                }
                _ => {
                    panic!("not a expression")
                }
            }
        }
    }

    // test call expression
    #[test]
    pub fn test_call_expression() {
        let input = "add(1,2 * 3,4 + 5);";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(
            1,
            program.statements.len(),
            "program statements does not contain {} statements",
            program.statements.len()
        );

        match program.statements[0].clone() {
            Statement::Expression(expr) => match expr.expression {
                Expr::Call {
                    function,
                    arguments,
                } => {
                    assert_eq!(function.string(), "add".to_string(),);
                    assert_eq!(arguments.len(), 3);
                    assert_eq!(arguments[0].string(), "1".to_string());
                    assert_eq!(arguments[1].string(), "(2 * 3)".to_string());
                    assert_eq!(arguments[2].string(), "(4 + 5)".to_string());
                }
                _ => {
                    panic!("expression is not a call expression")
                }
            },
            _ => {
                panic!("program.statements[0] is not an expression");
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
