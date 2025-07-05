use std::process::exit;

#[allow(unused_imports)]
use http_rs::run;

fn main() {
    match run() {
        Ok(()) => {
            print!("server exited")
        }
        Err(error) => {
            eprint!("{error}");
            exit(1);
        }
    }
}
