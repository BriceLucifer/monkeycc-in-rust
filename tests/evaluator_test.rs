mod evaluator_test {
    use monkeycc::{evaluator::eval, lexer::Lexer, object::Object, parser::Parser};

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
        ];

        for t in tests {
            let evaluated = test_eval(t.input);
            test_integer_object(evaluated, t.expect);
        }
    }

    pub fn test_eval(input: &str) -> Object {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        return eval(program);
    }

    pub fn test_integer_object(obj: Object, expected: i64) {
        match obj {
            Object::Integer(i) => assert_eq!(i, expected),
            _ => panic!("Object is not Integer"),
        }
    }
}
