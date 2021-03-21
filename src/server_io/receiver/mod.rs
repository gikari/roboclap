use std::sync::Arc;

mod parser;

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

            self.socket.recv_from(&mut buf).expect("Could not receive data!");

            // println!("Received data is: {}", std::str::from_utf8(&buf).unwrap());
            let message_str = std::str::from_utf8(&buf)
                .expect("Could not transform datagram to utf-8 string!").trim_end_matches(char::from(0));

            let parsed_expr = match lexpr::from_str(message_str) {
                Ok(value) => value,
                Err(error) => {
                    println!("Oops, cannot parse that s-expression: {}\n The error: {}", message_str, error);
                    panic!();
                }
            };

            parser::parse_into_struct(parsed_expr);
        }
    }

}

