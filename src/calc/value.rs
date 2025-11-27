use crate::calc::common::{Float, Integer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Value {
    Int(Integer),
    Float(Float),
}

impl Value {
    #[allow(dead_code)]
    pub fn as_int(&self) -> Integer {
        match self {
            Value::Int(i) => *i,
            Value::Float(f) => *f as Integer,
        }
    }

    pub fn as_float(&self) -> Float {
        match self {
            Value::Int(i) => *i as Float,
            Value::Float(f) => *f,
        }
    }

    pub fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (a, b) => Value::Float(a.as_float() + b.as_float()),
        }
    }

    pub fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            (a, b) => Value::Float(a.as_float() - b.as_float()),
        }
    }

    pub fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            (a, b) => Value::Float(a.as_float() * b.as_float()),
        }
    }

    pub fn div(self, rhs: Value) -> Value {
        Value::Float(self.as_float() / rhs.as_float())
    }

    pub fn rem(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
            (a, b) => Value::Float(a.as_float() % b.as_float()),
        }
    }

    pub fn pow(self, rhs: Value) -> Value {
        Value::Float(self.as_float().powf(rhs.as_float()))
    }

    pub fn neg(self) -> Value {
        match self {
            Value::Int(i) => Value::Int(-i),
            Value::Float(f) => Value::Float(-f),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(v) => write!(f, "Value::INT({})", v),
            Value::Float(v) => write!(f, "Value::FLOAT({})", v),
        }
    }
}
