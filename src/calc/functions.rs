use std::collections::HashMap;


use crate::calc::common::{Float, Integer};
use crate::calc::eval::{EvalResult, EvalErr};
use crate::calc::value::Value;


pub type FuncArg = [Value];
pub type FuncHandle = fn (&FuncArg) -> EvalResult;


pub fn builtin_funcs() -> HashMap<String, FuncHandle> {
    let mut hashmap: HashMap<String, FuncHandle> = HashMap::new();
    hashmap.insert("sin".into(), sin);
    hashmap.insert("cos".into(), cos);
    hashmap.insert("tan".into(), tan);
    hashmap.insert("min".into(), min);
    hashmap.insert("max".into(), max);
    hashmap.insert("avg".into(), avg);
    hashmap.insert("ceil".into(), ceil);
    hashmap.insert("floor".into(), floor);
    hashmap.insert("log".into(), log);
    return hashmap;
}

pub fn sin(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1, got {}", args.len());
        return Result::Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = &(args[0]);
    Result::Ok(Value::Float(x.as_float().sin()))
}

pub fn cos(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1, got {}", args.len());
        return Result::Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = &(args[0]);
    Result::Ok(Value::Float(x.as_float().cos()))
}

pub fn tan(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1, got {}", args.len());
        return Result::Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = &(args[0]);
    Result::Ok(Value::Float(x.as_float().tan()))
}

pub fn min(args: &FuncArg) -> EvalResult {
    if args.len() < 2 {
        let msg = format!("expected at least 2 values, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let mut min_val = &args[0];
    let mut min_float = args[0].as_float();

    for v in args.iter().skip(1) {
        let f = v.as_float();
        if f < min_float {
            min_float = f;
            min_val = v;
        }
    }

    Result::Ok((*min_val).clone())
}

pub fn max(args: &FuncArg) -> EvalResult {
    if args.len() < 2 {
        let msg = format!("expected at least 2 values, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let mut max_val = &args[0];
    let mut max_float = args[0].as_float();

    for v in args.iter().skip(1) {
        let f = v.as_float();
        if f > max_float {
            max_float = f;
            max_val = v;
        }
    }

    Result::Ok((*max_val).clone())
}

pub fn avg(args: &FuncArg) -> EvalResult {
    if args.is_empty() {
        return Err(EvalErr::IncorrectArgumentCount(
            "expected at least 1 value".into(),
        ));
    }

    let sum: Float = args.iter().map(|v| v.as_float()).sum();
    let avg = sum / args.len() as Float;

    if avg.fract() == 0.0 {
        return Result::Ok(Value::Int(avg as Integer));
    }

    Result::Ok(Value::Float(avg))
}

pub fn ceil(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1 value, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = args[0].as_float();
    let c = x.ceil();

    if c.fract() == 0.0 {
        return Ok(Value::Int(c as Integer));
    }

    Result::Ok(Value::Float(c))
}

pub fn floor(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1 value, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = args[0].as_float();
    let f = x.floor();

    if f.fract() == 0.0 {
        return Result::Ok(Value::Int(f as Integer));
    }

    Result::Ok(Value::Float(f))
}

pub fn log(args: &FuncArg) -> EvalResult {
    if args.len() != 2 {
        let msg = format!("expected 2, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let base = args[0].as_float();
    let value = args[1].as_float();

    if base <= 0.0 || base == 1.0 {
        return Err(EvalErr::InvalidArgument(
            "log base must be > 0 and != 1".into(),
        ));
    }
    if value <= 0.0 {
        return Err(EvalErr::InvalidArgument("log value must be > 0".into()));
    }

    let result = value.log(base);
    if result.fract() == 0.0 {
        return Result::Ok(Value::Int(result as Integer));
    }

    Result::Ok(Value::Float(result))
}
