use std::collections::HashMap;

pub struct ParamMessage {
    map: HashMap<String, Param>,
}

enum Param {
    Int(u32),
    Float(f64),
    String(String),
}

impl ParamMessage {
    pub fn from(sexpr: lexpr::Value) -> ParamMessage {
        ParamMessage {
            map: Default::default()
        }
    }
}