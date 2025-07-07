use std::{
    io::{Read},
    net::{TcpListener, TcpStream},
    thread,
};

use anyhow::{Context, Result};

mod request;
mod method;
mod response;
mod routes;

use request::parse_raw_request;
use response::send_response;

pub fn run(port: Option<u32>, directory: Option<String>) -> Result<()> {
    let port = port.unwrap_or(4221);
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).context("binding to address")?;

    println!("Listening on {addr}");

    for stream in listener.incoming() {
        // Handle stream errors here
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to accept connection: {e:?}");
                continue;
            }
        };  

        let directory = directory.clone();

        thread::spawn(move || {
            if let Err(e) = handle_connection(stream, directory) {
                eprintln!("Error handling connection: {e:?}");
            }
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, directory: Option<String>) -> Result<()> {
    let raw_request = read_stream(&mut stream).context("reading stream")?;
    let request = parse_raw_request(raw_request).context("parsing raw request")?;
    let response = routes::router(request, directory).context("routing request")?;
    send_response(response, &mut stream).context("sending response")?;
    Ok(())
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut request = vec![];

    loop {
        const BUFFER_SIZE: usize = 1024;
        let mut chunk = [0_u8; BUFFER_SIZE];
        let how_many_reads = stream.read(&mut chunk).context("reading request chunk")?;

        if how_many_reads == 0 {
            break;
        }

        request.extend_from_slice(&chunk[..how_many_reads]);

        if how_many_reads < BUFFER_SIZE {
            break;
        }
    }
    Ok(request)
}
