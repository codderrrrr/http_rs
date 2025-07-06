use anyhow::{Context, Ok, Result};
use bytes::{buf::Reader, Buf};

use crate::method::Method;
use std::{collections::HashMap, io::{BufRead}};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: Headers
}

pub fn parse_raw_request(request: Vec<u8>) ->  Result<Request> {
    let mut reader = request.reader();
    let method = parse_method_from_request(&mut reader).context("parsing method")?;
    let path = parse_path_from_request(&mut reader).context("parsing path")?;
    let protocol = parse_protocol_from_request(&mut reader).context("parsing protocol system from request")?;
    let headers = parse_headers_from_request(&mut reader).context("parsing header from request")?;
    Ok(Request { method, path, protocol, headers })

}

fn parse_path_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    const SPACE: u8 = b' ';
    let mut path_bytes = vec![];

    request.read_until(SPACE,  &mut path_bytes).context("parsing path")?;
    Ok(String::from_utf8(path_bytes).context("converting path bytes to string")?.trim().to_owned())
}

fn parse_method_from_request(request: &mut Reader<&[u8]>) -> Result<Method> {
    const SPACE: u8 = b' ';
    let mut method = vec![];

    request.read_until(SPACE,  &mut method).context("parsing path")?;
    Method::try_from(method)
}

fn parse_protocol_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    let mut protocol = String::new();

    request.read_line(&mut protocol).context("reading protocol bytes")?;
    Ok(protocol.trim().to_owned())
}

pub type Headers = HashMap<String, String>;
fn parse_headers_from_request(request: &mut Reader<&[u8]>) -> Result<Headers> {
    let mut headers = HashMap::new();

    loop {
        let mut raw_header = String::new();
        request.read_line(&mut raw_header).context("Reading header bytes")?;
        let header = raw_header.trim();

        if header.is_empty() {
            break;
        }

        let mut header_parts = raw_header.splitn(2, ':');
        let header_name = header_parts.next().map(|header: &str| header.trim().to_lowercase());
        let header_value = header_parts.next().map(|header: &str| header.trim().to_owned());

        if header_name.is_none() || header_value.is_none() {
            continue;
        }

        headers.insert(header_name.unwrap(), header_value.unwrap());
    }
    Ok(headers)
}