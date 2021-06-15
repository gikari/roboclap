use std::sync::Arc;

pub struct Commander {
    socket: Arc<std::net::UdpSocket>,
}

impl Commander {
    pub fn new(socket: Arc<std::net::UdpSocket>) -> Self {
        Self { socket }
    }

    pub fn init_player(&self, config: &crate::Config) {
        println!("Outbox thread is sending data...");

        // This is a default server address
        let server_address = format!("{}:{}", config.server_address, config.server_port);

        let message = format!(
            "(init {} (version {}))",
            config.team_name, config.server_protocol_version
        );
        let byte_array = message.as_bytes();

        self.socket
            .send_to(byte_array, server_address)
            .expect("Cannot send message!");

        println!("Message sent!");
    }
}
