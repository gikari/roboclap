use crate::message::MessageEnum;
use crate::message::Message;

pub fn parse_into_struct(parsed_expr: lexpr::Value) -> MessageEnum {
    match parsed_expr {
        lexpr::Value::Cons(ref value) => {
            let message_name = value.car().as_symbol().unwrap();
            match message_name {
                "sense_body" => MessageEnum::SenseBody(Message::from_sexpr(parsed_expr).unwrap()),
                "see" => MessageEnum::See(Message::from_sexpr(parsed_expr).unwrap()),
                "hear" => MessageEnum::Hear(Message::from_sexpr(parsed_expr).unwrap()),
                "server_param" => MessageEnum::ServerParam(Message::from_sexpr(parsed_expr).unwrap()),
                "player_param" => MessageEnum::PlayerParam(Message::from_sexpr(parsed_expr).unwrap()),
                "player_type" => MessageEnum::PlayerType(Message::from_sexpr(parsed_expr).unwrap()),
                "init" => MessageEnum::Init(Message::from_sexpr(parsed_expr).unwrap()),
                &_ => panic!("Unknown message {}!", message_name),
            }
        }
        _ => panic!(),
    }
}
