pub struct Hear {
    time: u8,
    sender: Sender,
    direction: Option<f64>,
    message: String,
}

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