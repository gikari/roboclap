mod server_io;
mod message;

fn main() -> std::io::Result<()> {
    // Create Server I/O threads
    let server_io_threads = server_io::init_threads();

    // TODO: Create thinking thread

    server_io_threads.sender_thread.join().unwrap();
    server_io_threads.receiver_thread.join().unwrap();

    Ok(())
}
