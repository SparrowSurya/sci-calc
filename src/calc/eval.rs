use crate::calc::nodes::{Atom, BinOp, Expr, UnOp};
use crate::calc::value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum EvalErr {
    FuncNotExists(String),
    IncorrectArgumentCount(String),
    ConstNotExists(String),
}

pub fn eval(expr: &Expr) -> Result<Value, EvalErr> {
    eval_expr(expr)
}

fn eval_expr(expr: &Expr) -> Result<Value, EvalErr> {
    match expr {
        Expr::Atom(a) => Result::Ok(eval_atom(a)?),
        Expr::UnOp(op, e) => Result::Ok(eval_unop(op, e)?),
        Expr::BinOp(op, lhs, rhs) => Result::Ok(eval_binop(op, lhs, rhs)?),
    }
}

fn eval_unop(op: &UnOp, expr: &Expr) -> Result<Value, EvalErr> {
    match op {
        UnOp::Neg => Result::Ok(eval_expr(expr)?.neg()),
        UnOp::Pos => Result::Ok(eval_expr(expr)?),
    }
}

fn eval_binop(op: &BinOp, lhs: &Expr, rhs: &Expr) -> Result<Value, EvalErr> {
    let lvalue = eval_expr(lhs)?;
    let rvalue = eval_expr(rhs)?;

    match op {
        BinOp::Plus => Result::Ok(lvalue.add(rvalue)),
        BinOp::Minus => Result::Ok(lvalue.sub(rvalue)),
        BinOp::Mul => Result::Ok(lvalue.mul(rvalue)),
        BinOp::Div => Result::Ok(lvalue.div(rvalue)),
        BinOp::Mod => Result::Ok(lvalue.rem(rvalue)),
        BinOp::Pow => Result::Ok(lvalue.pow(rvalue)),
    }
}

fn eval_atom(atom: &Atom) -> Result<Value, EvalErr> {
    match atom {
        Atom::Int(i) => Result::Ok(Value::Int(*i)),
        Atom::Float(f) => Result::Ok(Value::Float(*f)),
        Atom::Const(name) => Result::Ok(eval_const(name.to_string())?),
        Atom::Func(name, args) => {
            let values: Vec<Value> = args.iter()
                .map(|e| eval_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            Result::Ok(eval_func(name.to_string(), &values)?)
        }
    }
}

fn eval_const(name: String) -> Result<Value, EvalErr> {
    match name.to_lowercase().as_str() {
        "pi" => Result::Ok(Value::Float(std::f64::consts::PI)),
        "e" => Result::Ok(Value::Float(std::f64::consts::E)),
        _ => Result::Err(EvalErr::ConstNotExists(name)),
    }
}

fn eval_func(name: String, args: &[Value]) -> Result<Value, EvalErr> {
    match name.to_lowercase().as_str() {
        _ => Result::Err(EvalErr::FuncNotExists(name)),
    }
}
