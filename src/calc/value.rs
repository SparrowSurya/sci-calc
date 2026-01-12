use crate::calc::common::{Float, Integer};
use crate::calc::context::Context;
use crate::calc::eval::EvalErr;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Value {
    Int(Integer),
    Float(Float),
}

impl Value {
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

    pub fn not(self, ctx: &Context) -> Result<Value, EvalErr> {
        if let Value::Int(i) = self {
            return Result::Ok(Value::Int(!i));
        }

        if ctx.allow_floating_bitwise_operations {
            return Result::Ok(Value::Int(self.as_int()));
        }

        Result::Err(EvalErr::InvalidFloatingPointOperation(
            "Bitwise operator cannot be operated on floating point numbers".to_string()
        ))
    }

    pub fn and(self, rhs: Value, ctx: &Context) -> Result<Value, EvalErr> {
        let allow_fp_bw_ops = ctx.allow_floating_bitwise_operations;
        match (&self, &rhs) {
            (&Value::Int(i1), &Value::Int(i2)) => Result::Ok(Value::Int(i1 & i2)),
            (&Value::Int(i), &Value::Float(_)) if allow_fp_bw_ops => Result::Ok(Value::Int(i & rhs.as_int())),
            (&Value::Float(_), &Value::Int(i)) if allow_fp_bw_ops => Result::Ok(Value::Int(self.as_int() & i)),
            (&Value::Float(_), &Value::Float(_)) if allow_fp_bw_ops => Result::Ok(Value::Int(self.as_int() & rhs.as_int())),
            _ => Result::Err(EvalErr::InvalidFloatingPointOperation(
                "Bitwise operator cannot be operated on floating point numbers".to_string()
            ))
        }
    }

    pub fn or(self, rhs: Value, ctx: &Context) -> Result<Value, EvalErr> {
        let allow_fp_bw_ops = ctx.allow_floating_bitwise_operations;
        match (&self, &rhs) {
            (&Value::Int(i1), &Value::Int(i2)) => Result::Ok(Value::Int(i1 | i2)),
            (&Value::Int(i), &Value::Float(_)) if allow_fp_bw_ops => Result::Ok(Value::Int(i | rhs.as_int())),
            (&Value::Float(_), &Value::Int(i)) if allow_fp_bw_ops => Result::Ok(Value::Int(self.as_int() | i)),
            (&Value::Float(_), &Value::Float(_)) if allow_fp_bw_ops => Result::Ok(Value::Int(self.as_int() | rhs.as_int())),
            _ => Result::Err(EvalErr::InvalidFloatingPointOperation(
                "Bitwise operator cannot be operated on floating point numbers".to_string()
            ))
        }
    }

    pub fn xor(self, rhs: Value, ctx: &Context) -> Result<Value, EvalErr> {
        let allow_fp_bw_ops = ctx.allow_floating_bitwise_operations;
        match (&self, &rhs) {
            (&Value::Int(i1), &Value::Int(i2)) => Result::Ok(Value::Int(i1 ^ i2)),
            (&Value::Int(i), &Value::Float(_)) if allow_fp_bw_ops => Result::Ok(Value::Int(i ^ rhs.as_int())),
            (&Value::Float(_), &Value::Int(i)) if allow_fp_bw_ops => Result::Ok(Value::Int(self.as_int() ^ i)),
            (&Value::Float(_), &Value::Float(_)) if allow_fp_bw_ops => Result::Ok(Value::Int(self.as_int() ^ rhs.as_int())),
            _ => Result::Err(EvalErr::InvalidFloatingPointOperation(
                "Bitwise operator cannot be operated on floating point numbers".to_string()
            ))
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
