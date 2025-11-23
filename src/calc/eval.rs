use crate::calc::nodes::{Atom, BinOp, Expr, UnOp};
use crate::calc::value::Value;

pub fn eval(expr: &Expr) -> Value {
    eval_expr(expr)
}

fn eval_expr(expr: &Expr) -> Value {
    match expr {
        Expr::ATOM(a) => eval_atom(a),
        Expr::UNOP(op, e) => eval_unop(op, e),
        Expr::BINOP(op, lhs, rhs) => eval_binop(op, lhs, rhs),
    }
}

fn eval_unop(op: &UnOp, expr: &Expr) -> Value {
    match op {
        UnOp::NEG => eval_expr(expr).neg(),
        UnOp::POS => eval_expr(expr),
    }
}

fn eval_binop(op: &BinOp, lhs: &Expr, rhs: &Expr) -> Value {
    let l = eval_expr(lhs);
    let r = eval_expr(rhs);

    match op {
        BinOp::PLUS => l.add(r),
        BinOp::MINUS => l.sub(r),
        BinOp::MUL => l.mul(r),
        BinOp::DIV => l.div(r),
        BinOp::MOD => l.rem(r),
        BinOp::POW => l.pow(r),
    }
}

fn eval_atom(atom: &Atom) -> Value {
    match atom {
        Atom::INT(i) => Value::INT(*i),
        Atom::FLOAT(f) => Value::FLOAT(*f),
        Atom::CONST(name) => eval_const(name.to_string()),
        Atom::FUNC(name, args) => {
            let evaled_args: Vec<Value> = args.iter().map(|e| eval_expr(e)).collect();
            eval_func(name.to_string(), &evaled_args)
        }
    }
}

fn eval_const(name: String) -> Value {
    match name.to_lowercase().as_str() {
        "pi" => Value::FLOAT(std::f64::consts::PI),
        "e" => Value::FLOAT(std::f64::consts::E),
        _ => panic!("Unknown constant: {}", name),
    }
}

fn eval_func(name: String, args: &[Value]) -> Value {
    match name.to_lowercase().as_str() {
        _ => panic!("Unknown function: {}", name),
    }
}
