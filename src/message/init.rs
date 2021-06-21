use std::convert::TryFrom;

use crate::message::ParsingError;
use crate::message::ParsingError::BadSExpr;

pub struct Init {
    pub side: Side,
    pub player_number: u8,
    pub player_mode: String,
}

#[derive(PartialEq, Debug)]
pub enum Side {
    Left,
    Right,
}

impl TryFrom<lexpr::Value> for Init {
    type Error = ParsingError;

    fn try_from(sexpr: lexpr::Value) -> Result<Self, Self::Error> {
        // "(init (l (2 (before_kick_off))))"
        let name_cons = sexpr.as_cons().ok_or(BadSExpr)?;
        let side_cons = name_cons.cdr().as_cons().ok_or(BadSExpr)?;
        let player_number_cons = side_cons.cdr().as_cons().ok_or(BadSExpr)?;
        let game_mode_cons = player_number_cons.cdr().as_cons().ok_or(BadSExpr)?;

        let side_token = side_cons.car().as_symbol().ok_or(BadSExpr)?;

        let side = match side_token {
            "l" => Side::Left,
            "r" => Side::Right,
            _ => panic!(),
        };

        let player_number = player_number_cons.car().as_i64().ok_or(BadSExpr)?;

        let game_mode = game_mode_cons.car().as_symbol().ok_or(BadSExpr)?;

        Ok(Init {
            side,
            player_number: player_number as u8,
            player_mode: game_mode.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::message::init::{Init, Side};

    #[test]
    fn from_sexpr() {
        let sexpr_str = "(init l 2 before_kick_off)";
        let sexpr = lexpr::from_str(sexpr_str).unwrap();

        let init_message = Init::try_from(sexpr).unwrap();

        assert_eq!(init_message.side, Side::Left);
        assert_eq!(init_message.player_number, 2);
        assert_eq!(init_message.player_mode, "before_kick_off");
    }
}
