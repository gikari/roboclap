use std::sync::Arc;

pub struct Sender {
    socket: Arc<std::net::UdpSocket>,
}

impl Sender {
    pub fn new(socket: Arc<std::net::UdpSocket>) -> Sender {
        let sender = Sender { socket };
        sender
    }

    pub fn init_player(&self) {
        println!("Outbox thread is sending data...");

        // This is a default server address
        let server_address = "127.0.0.1:6000";

        const PROTOCOL_VERSION: i8 = 16;
        let team_name = "RustTeam";

        let message = format!("(init {} (version {}))", team_name, PROTOCOL_VERSION);
        let byte_array = message.as_bytes();

        self.socket
            .send_to(byte_array, server_address)
            .expect("Could not sent message!");

        println!("Message sent!");
    }
}
