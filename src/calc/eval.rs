use crate::calc::nodes::{Atom, BinOp, Expr, UnOp};
use crate::calc::value::Value;
use crate::calc::context::Context;


#[derive(Debug, Clone, PartialEq)]
pub enum EvalErr {
    FuncNotExists(String),
    IncorrectArgumentCount(String),
    ConstNotExists(String),
    InvalidArgument(String),
}

pub fn eval(ctx: &Context, expr: &Expr) -> Result<Value, EvalErr> {
    eval_expr(ctx, expr)
}

fn eval_expr(ctx: &Context, expr: &Expr) -> Result<Value, EvalErr> {
    match expr {
        Expr::Atom(a) => Result::Ok(eval_atom(ctx, a)?),
        Expr::UnOp(op, e) => Result::Ok(eval_unop(ctx, op, e)?),
        Expr::BinOp(op, lhs, rhs) => Result::Ok(eval_binop(ctx, op, lhs, rhs)?),
    }
}

fn eval_unop(ctx: &Context, op: &UnOp, expr: &Expr) -> Result<Value, EvalErr> {
    match op {
        UnOp::Neg => Result::Ok(eval_expr(ctx, expr)?.neg()),
        UnOp::Pos => Result::Ok(eval_expr(ctx, expr)?),
    }
}

fn eval_binop(ctx: &Context, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Result<Value, EvalErr> {
    let lvalue = eval_expr(ctx, lhs)?;
    let rvalue = eval_expr(ctx, rhs)?;

    match op {
        BinOp::Plus => Result::Ok(lvalue.add(rvalue)),
        BinOp::Minus => Result::Ok(lvalue.sub(rvalue)),
        BinOp::Mul => Result::Ok(lvalue.mul(rvalue)),
        BinOp::Div => Result::Ok(lvalue.div(rvalue)),
        BinOp::Mod => Result::Ok(lvalue.rem(rvalue)),
        BinOp::Pow => Result::Ok(lvalue.pow(rvalue)),
    }
}

fn eval_atom(ctx: &Context, atom: &Atom) -> Result<Value, EvalErr> {
    match atom {
        Atom::Int(i) => Result::Ok(Value::Int(*i)),
        Atom::Float(f) => Result::Ok(Value::Float(*f)),
        Atom::Const(name) => Result::Ok(eval_const(ctx, name.to_string())?),
        Atom::Func(name, args) => {
            let values: Vec<Value> = args.iter()
                .map(|e| eval_expr(ctx, e))
                .collect::<Result<Vec<_>, _>>()?;
            Result::Ok(eval_func(ctx, name.to_string(), &values)?)
        }
    }
}

fn eval_const(ctx: &Context, name: String) -> Result<Value, EvalErr> {
    ctx.consts
        .get(name.to_lowercase().as_str())
        .cloned()
        .ok_or_else(|| EvalErr::ConstNotExists(name))
}

fn eval_func(ctx: &Context, name: String, args: &[Value]) -> Result<Value, EvalErr> {
    ctx.funcs
        .get(name.to_lowercase().as_str())
        .ok_or_else(|| EvalErr::FuncNotExists(name.clone()))
        .and_then(|f| f(args))
}
