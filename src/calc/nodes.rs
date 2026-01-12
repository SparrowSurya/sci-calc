use crate::calc::common::{Float, Integer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum UnOp {
    Pos,
    Neg,
    Not,
}

#[derive(Clone, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Xor,
}

#[derive(Clone, PartialEq)]
pub enum Expr {
    UnOp(UnOp, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Atom(Atom),
}

#[derive(Clone, PartialEq)]
pub enum Atom {
    Int(Integer),
    Float(Float),
    Const(String),
    Func(String, Vec<Expr>),
}

impl fmt::Debug for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
            UnOp::Pos => write!(f, "+"),
            UnOp::Not => write!(f, "~"),
        }
    }
}

impl fmt::Debug for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sym = match self {
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Pow => "**",
            BinOp::And => "&",
            BinOp::Or => "|",
            BinOp::Xor => "^",
        };
        write!(f, "{}", sym)
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::UnOp(op, expr) => {
                write!(f, "({:?}{:?})", op, expr)
            }
            Expr::BinOp(op, left, right) => {
                write!(f, "({:?} {:?} {:?})", left, op, right)
            }
            Expr::Atom(a) => write!(f, "{:?}", a),
        }
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Int(i) => write!(f, "{}", i),
            Atom::Float(fl) => write!(f, "{}", fl),
            Atom::Const(name) => write!(f, "{}", name),
            Atom::Func(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}
