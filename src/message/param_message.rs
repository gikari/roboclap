use std::collections::HashMap;

use lexpr::Value;

use crate::message::ParsingError::UnexpectedSExpr;
use crate::message::{Message, ParsingError};

pub struct ParamMessage {
    pub map: HashMap<String, Param>,
}

#[derive(Debug, PartialEq)]
pub enum Param {
    Int(u64),
    Float(f64),
    String(String),
}

impl Message for ParamMessage {
    fn from_sexpr(sexpr: lexpr::Value) -> Result<ParamMessage, ParsingError> {
        // (x_param (p v) (p v) (p v))

        let name_cons = sexpr.as_cons().unwrap();

        let mut param_map: HashMap<String, Param> = HashMap::new();

        let params_cons = name_cons.cdr().as_cons().unwrap();
        for value in params_cons.list_iter() {
            let value_as_cons = value.as_cons().unwrap();
            let param_name = value_as_cons.car().as_symbol().unwrap();
            let param_value = value_as_cons.cdr().as_cons().unwrap().car();

            let param = match param_value {
                Value::Number(value_num) => {
                    if value_num.is_u64() {
                        Param::Int(value_num.as_u64().unwrap())
                    } else {
                        Param::Float(value_num.as_f64().unwrap())
                    }
                }
                Value::Symbol(value_str) => Param::String(value_str.to_string()),
                _ => return Err(UnexpectedSExpr),
            };

            param_map.insert(param_name.to_string(), param);
        }

        Ok(ParamMessage { map: param_map })
    }
}

impl ParamMessage {
    pub fn get_int(&self, key: &str) -> &u64 {
        let value_enum = self.map.get(key).unwrap();
        match value_enum {
            Param::Int(v) => v,
            _ => panic!(),
        }
    }

    pub fn get_float(&self, key: &str) -> &f64 {
        let value_enum = self.map.get(key).unwrap();
        match value_enum {
            Param::Float(v) => v,
            _ => panic!(),
        }
    }

    pub fn get_str(&self, key: &str) -> &str {
        let value_enum = self.map.get(key).unwrap();
        match value_enum {
            Param::String(v) => v,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::param_message::ParamMessage;
    use crate::message::Message;

    #[test]
    fn from_player_param() {
        test_param_message_with_name("player_param");
    }

    #[test]
    fn from_server_param() {
        test_param_message_with_name("server_param");
    }

    #[test]
    fn from_player_type() {
        test_param_message_with_name("player_type");
    }

    fn test_param_message_with_name(name: &str) {
        let first_param_value = 10;
        let second_param_value = "xyz";
        let third_param_value = 14.25;

        let sexpr_str = format!(
            "({} \
                (param1 {}) \
                (param2 {}) \
                (param_with_underscore {}) \
            )",
            name, first_param_value, second_param_value, third_param_value
        );
        let sexpr = lexpr::from_str(sexpr_str.as_str()).unwrap();

        let param_message: ParamMessage = Message::from_sexpr(sexpr).unwrap();

        assert_eq!(param_message.map.len(), 3);
        assert_eq!(param_message.get_int("param1"), &first_param_value);
        assert_eq!(param_message.get_str("param2"), second_param_value);
        assert_eq!(
            param_message.get_float("param_with_underscore"),
            &third_param_value
        );
    }
}
