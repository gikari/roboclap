pub mod sense_body;
pub mod see;
pub mod hear;
pub mod param_message;
pub mod init;

pub enum Message {
    SenseBody(sense_body::SenseBody),
    See(see::See),
    Hear(hear::Hear),
    PlayerParam(param_message::ParamMessage),
    ServerParam(param_message::ParamMessage),
    PlayerType(param_message::ParamMessage),
    Init(init::Init),
}