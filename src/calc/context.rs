use crate::calc::eval::EvalErr;
use crate::calc::value::Value;
use std::collections::HashMap;

pub struct Context {
    pub consts: HashMap<&'static str, Value>,
    pub funcs: HashMap<&'static str, fn(&[Value]) -> Result<Value, EvalErr>>,
    pub allow_floating_bitwise_operations: bool,
}

impl Context {
    pub fn new(
        consts: HashMap<&'static str, Value>,
        funcs: HashMap<&'static str, fn(&[Value]) -> Result<Value, EvalErr>>,
        allow_floating_bitwise_operations: bool,
    ) -> Context {
        Context {
            consts,
            funcs,
            allow_floating_bitwise_operations
        }
    }
}
