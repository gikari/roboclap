use structopt::StructOpt;

fn main() {
    let config = roboclap::Config::from_args();

    if let Err(e) = roboclap::run(config) {
        eprintln!("Error occurred: {}",
                  e);
        std::process::exit(1);
    }
}
