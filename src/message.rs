pub mod init;
pub mod param_message;

#[allow(dead_code)]
pub enum ServerMessage {
    Init(init::Init),
    PlayerParam(param_message::ParamMessage),
    PlayerType(param_message::ParamMessage),
    ServerParam(param_message::ParamMessage),
}

#[derive(Debug)]
pub enum ParsingError {
    BadSExpr,
}

pub trait Message {
    fn from_sexpr(sexpr: lexpr::Value) -> Result<Self, ParsingError>
    where
        Self: Sized;
}
