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
    Other,
}

impl Hear {
    pub fn from(sexpr: lexpr::Value) -> Hear {
        Hear {
            time: 0,
            sender: Sender::OnlineCoachLeft,
            direction: None,
            message: "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::hear::{Sender, Hear};

    #[test]
    fn from__hear_from_left_online_coach() {
        test_typical_sender("online_coach_left", Sender::OnlineCoachLeft);
    }

    #[test]
    fn from__hear_from_right_online_coach() {
        test_typical_sender("online_coach_right", Sender::OnlineCoachRight);
    }

    #[test]
    fn from__hear_from_coach() {
        test_typical_sender("coach", Sender::Coach);
    }

    #[test]
    fn from__hear_from_referee() {
        test_typical_sender("referee", Sender::Referee);
    }

    #[test]
    fn from__hear_from_self() {
        test_typical_sender("self", Sender::_Self);
    }

    #[test]
    fn from__hear_from_direction() {
        let sexpr_str = "(hear 10 42 sample_text";
        let sexpr = lexpr::from_str(sexpr_str).unwrap();

        let hear_message = Hear::from(sexpr);

        assert_eq!(hear_message.time, 10);
        assert_eq!(hear_message.sender, Sender::Other);
        assert_eq!(hear_message.direction.unwrap(), 42 as f64);
        assert_eq!(hear_message.message, "sample_text");
    }

    fn test_typical_sender(sender_str: &str, sender_enum: Sender) {
        let sexpr_str = format!("(hear 10 {} sample_text", sender_str);
        let sexpr = lexpr::from_str(sexpr_str.as_str()).unwrap();

        let hear_message = Hear::from(sexpr);

        assert_eq!(hear_message.time, 10);
        assert_eq!(hear_message.sender, sender_enum);
        assert_eq!(hear_message.direction, None);
        assert_eq!(hear_message.message, "sample_text");
    }
}
