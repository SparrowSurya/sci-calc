use std::collections::HashMap;
use crate::calc::value::Value;


pub fn builtin_consts() -> HashMap<String, Value> {
    let mut consts: HashMap<String, Value> = HashMap::new();
    consts.insert("pi".into(), Value::Float(std::f64::consts::PI));
    consts.insert("e".into(), Value::Float(std::f64::consts::E));
    return consts;
}
