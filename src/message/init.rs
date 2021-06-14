use crate::message::{Message, ParsingError};

pub struct Init {
    pub side: Side,
    pub player_number: u8,
    pub player_mode: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

impl Message for Init {
    fn from_sexpr(sexpr: lexpr::Value) -> Result<Init, ParsingError> {
        // "(init (l (2 (before_kick_off))))"
        let name_cons = sexpr.as_cons().unwrap();
        let side_cons = name_cons.cdr().as_cons().unwrap();
        let player_number_cons = side_cons.cdr().as_cons().unwrap();
        let game_mode_cons = player_number_cons.cdr().as_cons().unwrap();

        let side_token = side_cons.car().as_symbol().unwrap();

        let side = match side_token {
            "l" => Side::Left,
            "r" => Side::Right,
            _ => panic!(),
        };

        let player_number = player_number_cons.car().as_i64().unwrap();

        let game_mode = game_mode_cons.car().as_symbol().unwrap();

        Ok(Init {
            side,
            player_number: player_number as u8,
            player_mode: game_mode.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::init::{Init, Side};
    use crate::message::Message;

    #[test]
    fn from() {
        let sexpr_str = "(init l 2 before_kick_off)";
        let sexpr = lexpr::from_str(sexpr_str).unwrap();

        let init_message: Init = Message::from_sexpr(sexpr).unwrap();

        assert_eq!(init_message.side, Side::Left);
        assert_eq!(init_message.player_number, 2);
        assert_eq!(init_message.player_mode, "before_kick_off");
    }
}
