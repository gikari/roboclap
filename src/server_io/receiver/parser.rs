use crate::message::Message;
use crate::message::sense_body::SenseBody;
use crate::message::see::See;
use crate::message::hear::Hear;
use crate::message::param_message::ParamMessage;
use crate::message::init::Init;

pub fn parse_into_struct(parsed_expr: lexpr::Value) -> Message {
    match parsed_expr {
        lexpr::Value::Cons(ref value) => {
            let message_name = value.car().as_symbol().unwrap();
            match message_name {
                "sense_body" => Message::SenseBody(SenseBody::from(parsed_expr)),
                "see" => Message::See(See::from(parsed_expr)),
                "hear" => Message::Hear(Hear::from(parsed_expr)),
                "server_param" => Message::ServerParam(ParamMessage::from(parsed_expr).unwrap()),
                "player_param" => Message::PlayerParam(ParamMessage::from(parsed_expr).unwrap()),
                "player_type" => Message::PlayerType(ParamMessage::from(parsed_expr).unwrap()),
                "init" => Message::Init(Init::from(parsed_expr)),
                &_ => panic!("Unknown message {}!", message_name),
            }
        }
        _ => panic!(),
    }
}
