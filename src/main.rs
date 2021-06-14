fn main() {
    let config = roboclap::Config {};
    if let Err(e) = roboclap::run(config) {
        eprintln!("Error occurred: {}", e);
        std::process::exit(1);
    }
}
