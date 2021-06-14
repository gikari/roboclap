use structopt::StructOpt;

mod message;
mod server_io;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "roboclap")]
pub struct Config {
    #[structopt(short = "a", long, default_value = "127.0.0.1")]
    server_address: String,

    #[structopt(short = "p", long, default_value = "6000")]
    server_port: u8,

    #[structopt(long, default_value = "16")]
    server_protocol_version: u8,

    #[structopt(short = "t", long, default_value = "RustTeam")]
    team_name: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_for_io_threads = config.clone();

    // Create Server I/O threads
    let server_io_threads = server_io::init_threads(config_for_io_threads)?;

    // TODO: Create thinking thread

    server_io_threads.sender_thread.join().unwrap();
    server_io_threads.receiver_thread.join().unwrap();

    Ok(())
}
