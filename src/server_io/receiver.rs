use std::sync::Arc;

pub struct Receiver {
    socket: Arc<std::net::UdpSocket>,
}

impl Receiver {
    pub fn new(socket: Arc<std::net::UdpSocket>) -> Receiver {
        Receiver { socket }
    }

    pub fn start_message_loop(&self) {
        println!("Inbox thread is receiving data...");

        loop {
            let mut buf = [0; 4096];

            self.socket
                .recv_from(&mut buf)
                .expect("Could not receive data!");

            let message_str = std::str::from_utf8(&buf)
                .unwrap()
                .trim_end_matches(char::from(0));

            println!("Received data is: {}", message_str);
        }
    }
}
