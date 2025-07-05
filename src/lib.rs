use std::{io::{BufRead, Read}, net::TcpStream};
use std::{io::Write, net::TcpListener};
use anyhow::{bail, Context, Result};
use bytes::{buf::Reader, Buf};
use std::fmt::Display;

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

fn parse_raw_request(request: Vec<u8>) ->  Result<Request> {
    let mut reader = request.reader();
    let method = parse_method_from_request(&mut reader).context("parsing method")?;
    let path = parse_path_from_request(&mut reader).context("parsing path")?;
    Ok(Request { method, path })

}

fn parse_method_from_request(request: &mut Reader<&[u8]>) -> Result<Method> {
    const SPACE: u8 = b' ';
    let mut method = vec![];

    request.read_until(SPACE,  &mut method).context("parsing path")?;
    Method::try_from(method)
}

fn parse_path_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    const SPACE: u8 = b' ';
    let mut path_bytes = vec![];

    request.read_until(SPACE,  &mut path_bytes).context("parsing path")?;
    Ok(String::from_utf8(path_bytes).context("converting path bytes to string")?.trim().to_owned())
}

#[derive(Debug)]
struct Request {
    method: Method,
    path: String,
}

#[derive(Debug)]
enum Method {
    Get = 0,
}

impl TryFrom<Vec<u8>> for Method{
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        let method_string = String::from_utf8(value).context("Converting bytes to method string")?;

        Ok(match method_string.to_uppercase().trim() {
            "GET" => Self::Get,
            _ => bail!("Unknown method"),
        })
    }
}

enum HttpCode {
    Ok, 
    NotFound,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (number, message) = match self {
            Self::Ok => (200, "Ok"),
            Self::NotFound => (404, "Not Found"),
        };

        write!(f, "{number} {message}")
    }
}