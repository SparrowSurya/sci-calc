use crate::calc::eval::EvalErr;
use crate::calc::value::Value;
use std::collections::HashMap;

pub struct Context {
    pub consts: HashMap<&'static str, Value>,
    pub funcs: HashMap<&'static str, fn(&[Value]) -> Result<Value, EvalErr>>,
}

impl Context {
    pub fn new(
        consts: HashMap<&'static str, Value>,
        funcs: HashMap<&'static str, fn(&[Value]) -> Result<Value, EvalErr>>,
    ) -> Context {
        Context { consts, funcs }
    }
}
