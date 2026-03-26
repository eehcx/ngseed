fn main() {
    if let Err(err) = ngseed::run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}
