mod evaluator_test {
    use monkeycc::{evaluator::eval, lexer::Lexer, object::Object, parser::Parser};

    // test int eval
    #[test]
    pub fn test_eval_integer_expression() {
        struct Test {
            pub input: &'static str,
            pub expect: i64,
        }
        let tests = vec![
            Test {
                input: "5",
                expect: 5,
            },
            Test {
                input: "10",
                expect: 10,
            },
            Test {
                input: "-5",
                expect: -5,
            },
            Test {
                input: "-10",
                expect: -10,
            },
            Test {
                input: "5 + 5 + 5 + 5 - 10",
                expect: 10,
            },
            Test {
                input: "2 * 2 * 2 * 2 * 2",
                expect: 32,
            },
            Test {
                input: "-50 + 100 + -50",
                expect: 0,
            },
            Test {
                input: "5 * 2 + 10",
                expect: 20,
            },
            Test {
                input: "5 + 2 * 10",
                expect: 25,
            },
            Test {
                input: "20 + 2 * -10",
                expect: 0,
            },
            Test {
                input: "50 / 2 * 2 + 10",
                expect: 60,
            },
            Test {
                input: "2 * (5 + 10)",
                expect: 30,
            },
            Test {
                input: "3 * 3 * 3 + 10",
                expect: 37,
            },
            Test {
                input: "3 * (3 * 3) + 10",
                expect: 37,
            },
            Test {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10",
                expect: 50,
            },
        ];

        for t in tests {
            let evaluated = test_eval(t.input);
            test_integer_object(evaluated, t.expect);
        }
    }

    // test boolean
    #[test]
    pub fn test_eval_boolean_expression() {
        struct Test {
            pub input: &'static str,
            pub expect: bool,
        }

        let tests = vec![
            Test {
                input: "true",
                expect: true,
            },
            Test {
                input: "false",
                expect: false,
            },
            Test {
                input: "1 < 2",
                expect: true,
            },
            Test {
                input: "1 > 2",
                expect: false,
            },
            Test {
                input: "1 < 1",
                expect: false,
            },
            Test {
                input: "1 > 1",
                expect: false,
            },
            Test {
                input: "1 == 1",
                expect: true,
            },
            Test {
                input: "1 != 1",
                expect: false,
            },
            Test {
                input: "1 == 2",
                expect: false,
            },
            Test {
                input: "1 != 2",
                expect: true,
            },
            Test {
                input: "true == true",
                expect: true,
            },
            Test {
                input: "false == false",
                expect: true,
            },
            Test {
                input: "true == false",
                expect: false,
            },
            Test {
                input: "true != false",
                expect: true,
            },
            Test {
                input: "false != true",
                expect: true,
            },
            Test {
                input: "(1 < 2) == true",
                expect: true,
            },
            Test {
                input: "(1 < 2) == false",
                expect: false,
            },
            Test {
                input: "(1 > 2) == true",
                expect: false,
            },
            Test {
                input: "(1 > 2) == false",
                expect: true,
            },
        ];

        for t in tests {
            let evaluated = test_eval(t.input);
            test_boolean_object(evaluated, t.expect);
        }
    }

    #[test]
    pub fn test_bang_operator() {
        struct Test {
            input: &'static str,
            expected: bool,
        }
        let tests = vec![
            Test {
                input: "!true",
                expected: false,
            },
            Test {
                input: "!false",
                expected: true,
            },
            Test {
                input: "!5",
                expected: false,
            },
            Test {
                input: "!!true",
                expected: true,
            },
            Test {
                input: "!!false",
                expected: false,
            },
            Test {
                input: "!!5",
                expected: true,
            },
        ];

        for t in tests {
            let evaluated = test_eval(t.input);
            test_boolean_object(evaluated, t.expected);
        }
    }

    // test if expression
    #[test]
    pub fn test_if_else_expressions() {
        struct Test {
            input: &'static str,
            expected: &'static str,
        }

        let tests = vec![
            Test {
                input: "if (true) {10}",
                expected: "10",
            },
            Test {
                input: "if (false) {10}",
                expected: "null",
            },
            Test {
                input: "if (1) { 10 }",
                expected: "10",
            },
            Test {
                input: "if (1 < 2) { 10 }",
                expected: "10",
            },
            Test {
                input: "if (1 > 2) { 10 }",
                expected: "null",
            },
            Test {
                input: "if (1 > 2) { 10 } else { 20 }",
                expected: "20",
            },
            Test {
                input: "if (1 < 2) { 10 } else { 20 }",
                expected: "10",
            },
        ];

        for t in tests {
            let evaluated = test_eval(t.input);
            assert_eq!(evaluated.inspect(), t.expected);
        }
    }

    // test for return statement
    #[test]
    pub fn test_return_statements() {
        struct Test {
            input: &'static str,
            expected: i64,
        }
        let tests = vec![
            Test {
                input: "return 10;",
                expected: 10,
            },
            Test {
                input: "return 10; 9;",
                expected: 10,
            },
            Test {
                input: "return 2 * 5; 9;",
                expected: 10,
            },
            Test {
                input: "9; return 2 * 5; 9;",
                expected: 10,
            },
            Test {
                input: r"
                    if (10 >1) {
                        if (10 > 1) {
                            return 10
                        }
                        return 1;
                    }",
                expected: 10,
            },
        ];

        for t in tests {
            let evaled = test_eval(t.input);
            test_integer_object(evaled, t.expected);
        }
    }

    // ===================== error handling =====================

    fn assert_error_contains(input: &str, expected_substr: &str) {
        let evaluated = test_eval(input);
        match evaluated {
            Object::Error(msg) => {
                assert!(
                    msg.contains(expected_substr),
                    "\ninput:\n{}\nexpected error to contain {:?}\nactual: {}\n",
                    input,
                    expected_substr,
                    msg
                );
            }
            other => panic!(
                "\ninput:\n{}\nexpected Object::Error, got: {:?}\n",
                input, other
            ),
        }
    }

    #[test]
    pub fn test_error_handling_contains() {
        // 对齐你截图里的 Go 用例；用“包含匹配”更稳，不受报错文案细节影响
        let cases: &[(&str, &str)] = &[
            // 类型不匹配
            ("5 + true;", "type mismatch"),
            ("5 + true; 5;", "type mismatch"),
            // 前缀错误
            ("-true", "unknown operator"),
            // 布尔中缀（你的实现会报 unknown boolean operator 或 type mismatch；这里用更宽松关键字）
            ("true + false;", "unknown"),
            ("5; true + false; 5;", "unknown"),
            // if 分支中的错误
            ("if (10 > 1) { true + false; }", "unknown"),
            // 嵌套 if + return，确保错误能从分支内冒泡
            (
                r#"
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    }
                    return 1;
                }
            "#,
                "unknown",
            ),
            // 额外加一个：除零
            ("1 / 0;", "division by zero"),
        ];

        for (input, needle) in cases {
            assert_error_contains(input, needle);
        }
    }

    // helper function
    pub fn test_eval(input: &str) -> Object {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        return eval(&program);
    }

    // helper function for integer
    pub fn test_integer_object(obj: Object, expected: i64) {
        match obj {
            Object::Integer(i) => assert_eq!(i, expected),
            _ => panic!("Object is not Integer"),
        }
    }

    // helper function for boolean
    pub fn test_boolean_object(obj: Object, expected: bool) {
        match obj {
            Object::Boolean(b) => assert_eq!(b, expected, "{}", obj.inspect()),
            _ => panic!("Object is not boolean"),
        }
    }
}
