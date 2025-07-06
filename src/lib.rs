use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use anyhow::{Context, Result};

mod request;
mod method;
mod response;

use request::parse_raw_request;
use response::HttpCode;

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").context("binding to address")?;

    for stream in listener.incoming() {
        let mut stream = stream.context("accepting incoming connection")?;

        let raw_request = read_stream(&mut stream).context("reading stream")?;
        let request = parse_raw_request(raw_request).context("parsing raw request")?;

        println!("Request Method: {:?}, Path: {}", request.method, request.path);

        let response_code = if request.path == "/" { HttpCode::Ok } else { HttpCode::NotFound };


        let response = format!("HTTP/1.1 {}\r\n\r\n", response_code);


        stream.write_all(response.as_bytes()).context("writing all response data")?;
        stream.flush().context("flushing write")?;
    }

    Ok(())
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut request = vec![];

    loop {
        const BUFFER_SIZE: usize = 1024;
        let mut chunk = [0_u8; BUFFER_SIZE];
        let how_many_reads = stream.read(&mut chunk).context("Reading request chunk")?;

        request.extend_from_slice(&chunk[..how_many_reads]);

        if how_many_reads < BUFFER_SIZE {
            break;
        }
    }
    Ok(request)
}
