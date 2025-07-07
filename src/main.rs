use std::process::exit;
use std::env;

use anyhow::{Context, Result};
use http_rs::run;

fn main() {
    // Get optional PORT from environment
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u32>().ok());

    // Parse CLI arguments for --directory
    let directory = get_directory().context("getting directory argument").unwrap_or_else(|e| {
        eprintln!("Error getting directory argument: {e}");
        exit(1);
    });

    let directory = directory.unwrap_or_else(|| {
        eprintln!("Error: --directory <path> is required");
        exit(1);
    });

    // Start server
    match run(port, Some(directory)) {
        Ok(()) => {
            println!("Server exited cleanly");
        }
        Err(error) => {
            eprintln!("Server error: {error}");
            exit(1);
        }
    }
}

fn get_directory() -> Result<Option<String>> {
    let mut args = env::args();
    args.next(); // skip binary name

    while let Some(arg) = args.next() {
        if arg == "--directory" {
            return Ok(args.next());
        }
    }

    Ok(None)
}
