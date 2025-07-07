use std::process::exit;
use std::env;

#[allow(unused_imports)]
use http_rs::run;

fn main() {
    let port = env::var("PORT").ok().and_then(|port |port.parse::<u32>().ok());

    match run(port) {
        Ok(()) => {
            print!("server exited")
        }
        Err(error) => {
            eprint!("{error}");
            exit(1);
        }
    }
}
