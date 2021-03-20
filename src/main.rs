mod server_io;

fn main() -> std::io::Result<()> {
    // Create Server I/O threads
    let server_io_threads = server_io::init_threads();

    // Create thinking thread
    // TODO

    server_io_threads.sender_thread.join().unwrap();
    server_io_threads.receiver_thread.join().unwrap();

    Ok(())
}
