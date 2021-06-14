pub mod hear;
pub mod init;
pub mod param_message;
pub mod see;
pub mod sense_body;

pub enum MessageEnum {
    SenseBody(sense_body::SenseBody),
    See(see::See),
    Hear(hear::Hear),
    PlayerParam(param_message::ParamMessage),
    ServerParam(param_message::ParamMessage),
    PlayerType(param_message::ParamMessage),
    Init(init::Init),
}

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedSExpr,
}

pub(crate) trait Message {
    fn from_sexpr(sexpr: lexpr::Value) -> Result<Self, ParsingError> where Self: Sized;
}
