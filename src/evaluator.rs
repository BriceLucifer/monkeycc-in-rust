use crate::{
    ast::{Expr, Program, Statement},
    object::Object,
    token::TokenType,
};

// eval function
pub fn eval(program: &Program) -> Object {
    let mut result = Object::Null;

    for stmt in &program.statements {
        if let Statement::Expression(expr_stmt) = stmt {
            // 把表达式求值单独封装；遇到不支持的表达式就返回 Null，但这里不覆盖 result
            let v = eval_expr(&expr_stmt.expression);
            if !matches!(v, Object::Null) {
                result = v;
            }
        }
    }

    result
}

// eval the Expr::*
fn eval_expr(e: &Expr) -> Object {
    match e {
        Expr::Integer(i) => Object::Integer(*i),
        Expr::Boolean(b) => Object::Boolean(*b),

        // 前缀：先对 right 递归求值（按引用传递，无 clone/move）
        Expr::Prefix { op, right } => {
            let rv = eval_expr(right);

            match op {
                TokenType::Bang => Object::Boolean(!is_truthy(&rv)),

                // 一元 -：仅对整数有效
                TokenType::Minus => match rv {
                    Object::Integer(i) => Object::Integer(-i),
                    _ => Object::Null,
                },

                // 一元 +：通常是 no-op，仅对整数有效
                TokenType::Plus => match rv {
                    Object::Integer(i) => Object::Integer(i),
                    _ => Object::Null,
                },

                _ => Object::Null,
            }
        }

        Expr::Infix { left, op, right } => {
            println!("{}", op);
            let left = eval_expr(left);
            let right = eval_expr(right);
            eval_infix_expression(op, left, right)
        }

        // 其它表达式暂不支持
        _ => Object::Null,
    }
}

// for check the object
fn is_truthy(o: &Object) -> bool {
    match o {
        Object::Boolean(b) => *b,
        Object::Integer(i) => *i != 0,
        Object::Null => false,
        _ => true,
    }
}

fn eval_infix_expression(op: &TokenType, left: Object, right: Object) -> Object {
    println!("{}", op);
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => match *op {
            // +
            TokenType::Plus => Object::Integer(l + r),
            // -
            TokenType::Minus => Object::Integer(l - r),
            // *
            TokenType::Asterisk => Object::Integer(l * r),
            // /
            TokenType::Slash => Object::Integer(l / r),
            // ==
            TokenType::Eq => Object::Boolean(l == r),
            // !=
            TokenType::NotEq => Object::Boolean(l != r),
            // >=
            TokenType::Ge => Object::Boolean(l >= r),
            // <=
            TokenType::Le => Object::Boolean(l <= r),
            // >
            TokenType::Gt => Object::Boolean(l > r),
            // <
            TokenType::Lt => Object::Boolean(l < r),
            _ => Object::Null,
        },
        (Object::Boolean(l), Object::Boolean(r)) => match *op {
            // ==
            TokenType::Eq => Object::Boolean(l == r),
            // !=
            TokenType::NotEq => Object::Boolean(l != r),
            _ => Object::Null,
        },
        _ => Object::Null,
    }
}
