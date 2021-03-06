use std::sync::Arc;

use structopt::StructOpt;

mod commander;
mod message;
mod sensor;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "roboclap")]
pub struct Config {
    #[structopt(short = "a", long, default_value = "127.0.0.1")]
    server_address: String,

    #[structopt(short = "p", long, default_value = "6000")]
    server_port: u16,

    #[structopt(long, default_value = "16")]
    server_protocol_version: u8,

    #[structopt(short = "t", long, default_value = "RustTeam")]
    team_name: String,
}

pub struct ThreadsPack {
    pub receiver_thread: std::thread::JoinHandle<()>,
    pub sender_thread: std::thread::JoinHandle<()>,
    pub thinking_thread: std::thread::JoinHandle<()>,
}

pub fn run(config: Config) -> anyhow::Result<()> {
    let threads_pack = init_main_threads(config)?;

    threads_pack.receiver_thread.join().unwrap();
    threads_pack.sender_thread.join().unwrap();
    threads_pack.thinking_thread.join().unwrap();

    Ok(())
}

fn init_main_threads(config: Config) -> anyhow::Result<ThreadsPack> {
    let socket_for_sender = Arc::new(std::net::UdpSocket::bind("0.0.0.0:0")?);
    let socket_for_receiver = socket_for_sender.clone();

    let config_for_sender = config;

    let threads = ThreadsPack {
        receiver_thread: std::thread::spawn(move || {
            let receiver = sensor::Sensor::new(socket_for_receiver);
            receiver.start_message_loop();
        }),
        sender_thread: std::thread::spawn(move || {
            let sender = commander::Commander::new(socket_for_sender);
            sender.init_player(&config_for_sender);
        }),
        thinking_thread: std::thread::spawn(move || {}),
    };

    Ok(threads)
}
