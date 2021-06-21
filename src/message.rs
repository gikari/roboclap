use thiserror::Error;

pub mod init;
pub mod param_message;

#[allow(dead_code)]
pub enum ServerMessage {
    Init(init::Init),
    PlayerParam(param_message::ParamMessage),
    PlayerType(param_message::ParamMessage),
    ServerParam(param_message::ParamMessage),
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("S-Expression structure is unexpected")]
    BadSExpr,
}
