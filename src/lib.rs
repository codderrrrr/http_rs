use std::{
    io::{Read},
    net::{TcpListener, TcpStream},
};

use anyhow::{Context, Result};

mod request;
mod method;
mod response;

mod routes;
use request::parse_raw_request;
use response::{send_response};

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").context("binding to address")?;

    for stream in listener.incoming() {
        let mut stream = stream.context("accepting incoming connection")?;

        let raw_request = read_stream(&mut stream).context("reading stream")?;
        let request = parse_raw_request(raw_request).context("parsing raw request")?;
        let response = routes::router(request).context("routing request")?;

        //let response_code = if request.path == "/" { HttpCode::Ok } else { HttpCode::NotFound };


        send_response(response, &mut stream)?;
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
