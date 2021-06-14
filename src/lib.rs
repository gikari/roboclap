use structopt::StructOpt;

mod message;
mod server_io;

#[derive(StructOpt)]
pub struct Config {

}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Create Server I/O threads
    let server_io_threads = server_io::init_threads();

    // TODO: Create thinking thread

    server_io_threads.sender_thread.join().unwrap();
    server_io_threads.receiver_thread.join().unwrap();

    Ok(())
}
