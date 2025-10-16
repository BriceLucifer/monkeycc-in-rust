use crate::{
    ast::{Expr, Program, Statement},
    object::Object,
    token::TokenType,
};

/* ========== error helpers ========== */

fn err<S: Into<String>>(msg: S) -> Object {
    Object::Error(msg.into())
}

fn is_error(o: &Object) -> bool {
    matches!(o, Object::Error(_))
}

/* ========== control flow carrier ========== */

#[derive(Debug, Clone)]
enum EvalFlow {
    Value(Object),  // 正常值
    Return(Object), // return 冒泡（以后在函数边界消化）
}

/* ========== public entry ========== */

pub fn eval(program: &Program) -> Object {
    match eval_statements(&program.statements) {
        EvalFlow::Value(v) | EvalFlow::Return(v) => v, // 顶层：先都当值输出
    }
}

/* ========== core ========== */

fn eval_statements(stmts: &Vec<Statement>) -> EvalFlow {
    let mut last = Object::Null;

    for stmt in stmts {
        match stmt {
            Statement::Block(block) => {
                match eval_statements(&block.statements) {
                    r @ EvalFlow::Return(_) => return r, // 冒泡
                    EvalFlow::Value(v) => {
                        if is_error(&v) {
                            return EvalFlow::Value(v);
                        } // 错误粘性传播
                        last = v;
                    }
                }
            }

            Statement::Expression(expr_stmt) => match eval_expr(&expr_stmt.expression) {
                r @ EvalFlow::Return(_) => return r,
                EvalFlow::Value(v) => {
                    if is_error(&v) {
                        return EvalFlow::Value(v);
                    }
                    last = v;
                }
            },

            Statement::Return(ret_stmt) => {
                let v = match eval_expr(&ret_stmt.return_value) {
                    EvalFlow::Return(v) => v,
                    EvalFlow::Value(v) => v,
                };
                return EvalFlow::Return(v); // 立刻冒泡
            }

            Statement::Let { .. } => {
                // 以后接环境 env.set(...)
            }

            Statement::None => {}
        }
    }

    EvalFlow::Value(last)
}

// 当 if 分支不是 Block（虽然很少见），用这个处理单个 Statement
fn eval_single_statement(stmt: &Statement) -> EvalFlow {
    match stmt {
        Statement::Block(b) => eval_statements(&b.statements),

        Statement::Expression(e) => match eval_expr(&e.expression) {
            r @ EvalFlow::Return(_) => r,
            EvalFlow::Value(v) => {
                if is_error(&v) {
                    EvalFlow::Value(v)
                } else {
                    EvalFlow::Value(v)
                }
            }
        },

        Statement::Return(r) => {
            let v = match eval_expr(&r.return_value) {
                EvalFlow::Return(v) => v,
                EvalFlow::Value(v) => v,
            };
            EvalFlow::Return(v)
        }

        Statement::Let { .. } | Statement::None => EvalFlow::Value(Object::Null),
    }
}

fn eval_expr(e: &Expr) -> EvalFlow {
    match e {
        Expr::Integer(i) => EvalFlow::Value(Object::Integer(*i)),
        Expr::Boolean(b) => EvalFlow::Value(Object::Boolean(*b)),

        // 前缀
        Expr::Prefix { op, right } => {
            let rv = match eval_expr(right) {
                r @ EvalFlow::Return(_) => return r,
                EvalFlow::Value(v) => v,
            };
            if is_error(&rv) {
                return EvalFlow::Value(rv);
            } // 错误上抛

            let out = match op {
                TokenType::Bang => Object::Boolean(!is_truthy(&rv)),
                TokenType::Minus => match rv {
                    Object::Integer(i) => Object::Integer(-i),
                    _ => err(format!("unknown operator: - {:?}", rv)),
                },
                TokenType::Plus => match rv {
                    Object::Integer(i) => Object::Integer(i),
                    _ => err(format!("unknown operator: + {:?}", rv)),
                },
                _ => err(format!("unknown prefix operator: {}", op)),
            };
            EvalFlow::Value(out)
        }

        // 中缀
        Expr::Infix { left, op, right } => {
            let lv = match eval_expr(left) {
                r @ EvalFlow::Return(_) => return r,
                EvalFlow::Value(v) => v,
            };
            if is_error(&lv) {
                return EvalFlow::Value(lv);
            }

            let rv = match eval_expr(right) {
                r @ EvalFlow::Return(_) => return r,
                EvalFlow::Value(v) => v,
            };
            if is_error(&rv) {
                return EvalFlow::Value(rv);
            }

            EvalFlow::Value(eval_infix_expression(op, lv, rv))
        }

        // if 表达式
        Expr::IfExpression {
            condition,
            consequence,
            alternative,
        } => {
            let cond = match eval_expr(condition) {
                r @ EvalFlow::Return(_) => return r,
                EvalFlow::Value(v) => v,
            };
            // 条件若出错，直接上抛；不要把错误当布尔用
            if is_error(&cond) {
                return EvalFlow::Value(cond);
            }

            let chosen: &Statement = if is_truthy(&cond) {
                consequence
            } else {
                alternative
            };

            match chosen {
                Statement::Block(b) => eval_statements(&b.statements),
                other => eval_single_statement(other),
            }
        }

        // 其它暂不支持
        _ => EvalFlow::Value(Object::Null),
    }
}

/* ========== helpers ========== */

fn is_truthy(o: &Object) -> bool {
    match o {
        Object::Boolean(b) => *b,
        Object::Integer(i) => *i != 0,
        Object::Null => false,
        _ => {
            // 理论上我们在进入 is_truthy 前就已拦截错误/其它类型
            debug_assert!(
                matches!(o, Object::Boolean(_) | Object::Integer(_) | Object::Null),
                "is_truthy called with non-boolean/integer/null: {:?}",
                o
            );
            false
        }
    }
}

fn eval_infix_expression(op: &TokenType, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => match *op {
            TokenType::Plus => Object::Integer(l + r),
            TokenType::Minus => Object::Integer(l - r),
            TokenType::Asterisk => Object::Integer(l * r),
            TokenType::Slash => {
                if r == 0 {
                    err("division by zero")
                } else {
                    Object::Integer(l / r)
                }
            }
            TokenType::Eq => Object::Boolean(l == r),
            TokenType::NotEq => Object::Boolean(l != r),
            TokenType::Ge => Object::Boolean(l >= r),
            TokenType::Le => Object::Boolean(l <= r),
            TokenType::Gt => Object::Boolean(l > r),
            TokenType::Lt => Object::Boolean(l < r),
            _ => err(format!("unknown integer operator: {}", op)),
        },

        (Object::Boolean(l), Object::Boolean(r)) => match *op {
            TokenType::Eq => Object::Boolean(l == r),
            TokenType::NotEq => Object::Boolean(l != r),
            _ => err(format!("unknown boolean operator: {}", op)),
        },

        // 注意这里绑定 (l, r) 才能在错误消息里使用
        (l, r) => err(format!("type mismatch: {:?} {} {:?}", l, op, r)),
    }
}
