use std::process::exit;

fn main() {
    if let Err(e) = rust_minicat::get_args().and_then(rust_minicat::run) {
        eprintln!("{}", e);
        exit(1);
    }
}
