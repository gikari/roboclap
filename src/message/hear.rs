use std::borrow::Borrow;

use lexpr::Value;

use crate::message::ParsingError::UnexpectedSExpr;
use crate::message::{Message, ParsingError};

pub struct Hear {
    time: u8,
    sender: Sender,
    direction: Option<f64>,
    message: String,
}

#[derive(Debug, Eq, PartialEq)]
enum Sender {
    OnlineCoachLeft,
    OnlineCoachRight,
    Coach,
    Referee,
    _Self,
    Player(u8),
    Other,
}

impl Message for Hear {
    fn from_sexpr(sexpr: lexpr::Value) -> Result<Hear, ParsingError> {
        let mut it = sexpr.list_iter().ok_or(UnexpectedSExpr)?;

        it.next(); // Skip name

        let time = it
            .next()
            .ok_or(UnexpectedSExpr)?
            .as_number()
            .ok_or(UnexpectedSExpr)?
            .as_u64()
            .ok_or(UnexpectedSExpr)? as u8;

        let second_item = it.next().ok_or(UnexpectedSExpr)?;
        let mut direction: Option<f64> = None;
        let mut sender = Sender::Other;
        match second_item {
            Value::Number(_direction) => {
                direction = Some(_direction.as_f64().ok_or(UnexpectedSExpr)?);
            }
            Value::Symbol(sender_str) => {
                sender = match sender_str.borrow() {
                    "online_coach_left" => Sender::OnlineCoachLeft,
                    "online_coach_right" => Sender::OnlineCoachRight,
                    "coach" => Sender::Coach,
                    "referee" => Sender::Referee,
                    "self" => Sender::_Self,
                    _ => Sender::Other,
                }
            }
            _ => return Err(UnexpectedSExpr),
        }

        if direction.is_some() {
            todo!("Parse with ")
        } else {
        }

        Ok(Hear {
            time,
            sender,
            direction,
            message: "".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::hear::{Hear, Sender};
    use crate::message::Message;

    #[test]
    fn from_hear_from_left_online_coach() {
        test_typical_sender("online_coach_left", Sender::OnlineCoachLeft);
    }

    #[test]
    fn from_hear_from_right_online_coach() {
        test_typical_sender("online_coach_right", Sender::OnlineCoachRight);
    }

    #[test]
    fn from_hear_from_coach() {
        test_typical_sender("coach", Sender::Coach);
    }

    #[test]
    fn from_hear_from_referee() {
        test_typical_sender("referee", Sender::Referee);
    }

    #[test]
    fn from_hear_from_self() {
        test_typical_sender("self", Sender::_Self);
    }

    #[test]
    fn from_hear_from_direction() {
        let sexpr_str = "(hear 10 42 sample_text)";
        let sexpr = lexpr::from_str(sexpr_str).unwrap();

        let hear_message: Hear = Message::from_sexpr(sexpr).unwrap();

        assert_eq!(hear_message.time, 10);
        assert_eq!(hear_message.sender, Sender::Other);
        assert_eq!(hear_message.direction.unwrap(), 42 as f64);
        assert_eq!(hear_message.message, "sample_text");
    }

    #[test]
    fn from_hear_from_our_player() {
        // (hear Time Dir our Unum Message)
        let sexpr_str = "(hear 10 42 our 1 sample_text)";
        let sexpr = lexpr::from_str(sexpr_str).unwrap();

        let hear_message: Hear = Message::from_sexpr(sexpr).unwrap();

        assert_eq!(hear_message.time, 10);
        assert_eq!(hear_message.sender, Sender::Other);
        assert_eq!(hear_message.direction.unwrap(), 42 as f64);
        assert_eq!(hear_message.message, "sample_text");
    }

    #[test]
    fn from_hear_from_enemy_player() {
        todo!();
        // (hear Time Dir opp Message)
        let sexpr_str = "(hear Time Dir our Message)";
        let sexpr = lexpr::from_str(sexpr_str).unwrap();

        let hear_message: Hear = Message::from_sexpr(sexpr).unwrap();

        assert_eq!(hear_message.time, 10);
        assert_eq!(hear_message.sender, Sender::Other);
        assert_eq!(hear_message.direction.unwrap(), 42 as f64);
        assert_eq!(hear_message.message, "sample_text");
    }

    fn test_typical_sender(sender_str: &str, sender_enum: Sender) {
        let sexpr_str = format!("(hear 10 {} sample_text)", sender_str);
        let sexpr = lexpr::from_str(sexpr_str.as_str()).unwrap();

        let hear_message: Hear = Message::from_sexpr(sexpr).unwrap();

        assert_eq!(hear_message.time, 10);
        assert_eq!(hear_message.sender, sender_enum);
        assert_eq!(hear_message.direction, None);
        assert_eq!(hear_message.message, "sample_text");
    }
}
