use std::collections::HashMap;


use crate::calc::common::{Integer, Float};
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
    hashmap.insert("deg".into(), deg);
    hashmap.insert("rad".into(), rad);
    hashmap.insert("fact".into(), fact);
    hashmap.insert("ncr".into(), ncr);
    hashmap.insert("npr".into(), npr);
    return hashmap;
}

pub fn sin(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1, got {}", args.len());
        return Result::Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = &(args[0]);
    Result::Ok(Value::auto(x.as_float().sin()))
}

pub fn cos(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1, got {}", args.len());
        return Result::Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = &(args[0]);
    Result::Ok(Value::auto(x.as_float().cos()))
}

pub fn tan(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1, got {}", args.len());
        return Result::Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let x = &(args[0]);
    Result::Ok(Value::auto(x.as_float().tan()))
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
    Result::Ok(Value::auto(avg))
}

pub fn ceil(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1 value, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    Result::Ok(Value::auto(args[0].as_float().ceil()))
}

pub fn floor(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1 value, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    Result::Ok(Value::auto(args[0].as_float().floor()))
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

    Result::Ok(Value::auto(value.log(base)))
}

pub fn deg(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1 value, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    Result::Ok(Value::auto(args[0].as_float().to_degrees()))
}

pub fn rad(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1 value, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    Result::Ok(Value::auto(args[0].as_float().to_radians()))
}

pub fn fact(args: &FuncArg) -> EvalResult {
    if args.len() != 1 {
        let msg = format!("expected 1 value, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    match args[0] {
        Value::Int(i) => {
            if i < 0 {
                let msg = format!("expected non-negative integer, got {}", i);
                return Result::Err(EvalErr::InvalidArgument(msg));
            } else if i == 0 {
                return Result::Ok(Value::Int(1 as Integer));
            }
            return Result::Ok(Value::Int((0..i).product()));
        },
        Value::Float(f) => {
            let msg = format!("expected non-negative integer, got {}", f);
            return Result::Err(EvalErr::InvalidArgument(msg));
        },
    }
}

pub fn ncr(args: &FuncArg) -> EvalResult {
    if args.len() != 2 {
        let msg = format!("expected 2, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let n = args[0].as_int();
    let r = args[1].as_int();
    if n < r {
        return Result::Ok(Value::zero());
    }

    let r = r.min(n-r);
    let mut result = 1;

    for i in 0..r {
        result = result * (n-i) / (i+1);
    }

    Result::Ok(Value::Int(result as Integer))
}

pub fn npr(args: &FuncArg) -> EvalResult {
    if args.len() != 2 {
        let msg = format!("expected 2, got {}", args.len());
        return Err(EvalErr::IncorrectArgumentCount(msg));
    }

    let n = args[0].as_int();
    let r = args[1].as_int();
    if n < r {
        return Result::Ok(Value::zero());
    }

    let mut result = 1;
    for i in 0..r {
        result = n-i;
    }

    Result::Ok(Value::Int(result as Integer))
}
