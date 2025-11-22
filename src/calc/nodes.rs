use crate::calc::common::{Float, Integer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum UnOp {
    POS,
    NEG,
}

#[derive(Clone, PartialEq)]
pub enum BinOp {
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    POW,
}

#[derive(Clone, PartialEq)]
pub enum Expr {
    UNOP(UnOp, Box<Expr>),
    BINOP(BinOp, Box<Expr>, Box<Expr>),
    ATOM(Atom),
}

#[derive(Clone, PartialEq)]
pub enum Atom {
    INT(Integer),
    FLOAT(Float),
    CONST(String),
    FUNC(String, Vec<Expr>),
}

impl fmt::Debug for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::NEG => write!(f, "-"),
            UnOp::POS => write!(f, "+"),
        }
    }
}

impl fmt::Debug for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sym = match self {
            BinOp::PLUS => "+",
            BinOp::MINUS => "-",
            BinOp::MUL => "*",
            BinOp::DIV => "/",
            BinOp::MOD => "%",
            BinOp::POW => "^",
        };
        write!(f, "{}", sym)
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::UNOP(op, expr) => {
                write!(f, "({:?}{:?})", op, expr)
            }

            Expr::BINOP(op, left, right) => {
                write!(f, "({:?} {:?} {:?})", left, op, right)
            }

            Expr::ATOM(a) => write!(f, "{:?}", a),
        }
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::INT(i) => write!(f, "{}", i),
            Atom::FLOAT(fl) => write!(f, "{}", fl),
            Atom::CONST(name) => write!(f, "{}", name),

            Atom::FUNC(name, args) => {
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
