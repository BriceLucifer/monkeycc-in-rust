#[cfg(test)]
mod ast_test {
    use monkeycc::ast::{Expr, Ident, Program, Statement};

    // 测试String功能是否正常
    #[test]
    fn test_string() {
        let mut program = Program {
            statements: Vec::new(),
        };
        let let_stmt = Statement::Let {
            name: Ident("myVar".to_string()),
            value: Expr::Ident(Ident("anotherVar".to_string())),
        };
        program.statements.push(let_stmt);
        assert_eq!("let myVar = anotherVar;".to_string(), program.string())
    }
}
