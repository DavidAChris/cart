fn main() {
    if let Err(err) = cart::get_args().and_then(cart::run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
