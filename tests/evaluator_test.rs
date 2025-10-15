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
