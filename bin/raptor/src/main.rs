fn main() {
    if let Err(err) = raptor::cli::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
