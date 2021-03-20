// Server I/O module

use std::sync::Arc;

mod receiver;
mod sender;

pub struct ThreadsPack {
    pub receiver_thread: std::thread::JoinHandle<()>,
    pub sender_thread: std::thread::JoinHandle<()>,
}

pub fn init_threads() -> ThreadsPack {
    println!("Initializing threads...");

    let socket_for_sender =
        Arc::new(std::net::UdpSocket::bind("0.0.0.0:0").expect("Could not bind to the address!"));
    let socket_for_receiver = socket_for_sender.clone();

    ThreadsPack {
        receiver_thread: std::thread::spawn(move || {
            let receiver = receiver::Receiver::new(socket_for_receiver);
            receiver.start_message_loop();
        }),
        sender_thread: std::thread::spawn(move || {
            let sender = sender::Sender::new(socket_for_sender);
            sender.init_player();
        }),
    }
}
