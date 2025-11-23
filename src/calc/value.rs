use crate::calc::common::{Float, Integer};
use std::fmt;

#[derive(PartialEq)]
pub enum Value {
    INT(Integer),
    FLOAT(Float),
}

impl Value {
    pub fn as_int(&self) -> Integer {
        match self {
            Value::INT(i) => *i,
            Value::FLOAT(f) => *f as Integer,
        }
    }

    pub fn as_float(&self) -> Float {
        match self {
            Value::INT(i) => *i as Float,
            Value::FLOAT(f) => *f,
        }
    }

    pub fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a + b),
            (a, b) => Value::FLOAT(a.as_float() + b.as_float()),
        }
    }

    pub fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a - b),
            (a, b) => Value::FLOAT(a.as_float() - b.as_float()),
        }
    }

    pub fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a * b),
            (a, b) => Value::FLOAT(a.as_float() * b.as_float()),
        }
    }

    pub fn div(self, rhs: Value) -> Value {
        Value::FLOAT(self.as_float() / rhs.as_float())
    }

    pub fn rem(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a % b),
            (a, b) => Value::FLOAT(a.as_float() % b.as_float()),
        }
    }

    pub fn pow(self, rhs: Value) -> Value {
        Value::FLOAT(self.as_float().powf(rhs.as_float()))
    }

    pub fn neg(self) -> Value {
        match self {
            Value::INT(i) => Value::INT(-i),
            Value::FLOAT(f) => Value::FLOAT(-f),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::INT(v) => write!(f, "Value::INT({})", v),
            Value::FLOAT(v) => write!(f, "Value::FLOAT({})", v),
        }
    }
}
