use crate::calc::functions::FuncHandle;
use crate::calc::value::Value;
use std::collections::HashMap;


pub struct Context {
    pub consts: HashMap<String, Value>,
    pub funcs: HashMap<String, FuncHandle>,
    pub allow_floating_bitwise_operations: bool,
}
