pub struct Init {
    pub side: Side,
    pub player_number: u8,
    pub player_mode: String,
}

pub enum Side {
    Left,
    Right
}

impl Init {
    pub fn from(sexpr: lexpr::Value) -> Init {
        Init {
            side: Side::Left,
            player_number: 0,
            player_mode: "".to_string()
        }
    }
}
