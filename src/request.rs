use anyhow::{Context, Result};
use bytes::{buf::Reader, Buf};
use std::{
    collections::HashMap,
    io::{BufRead, Read},
};

use crate::method::Method;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: Headers,
    pub body: Option<String>,
}

pub fn parse_raw_request(request: Vec<u8>) -> Result<Request> {
    let mut reader = request.reader();

    let method = parse_method_from_request(&mut reader).context("parsing method")?;
    let path = parse_path_from_request(&mut reader).context("parsing path")?;
    let protocol = parse_protocol_from_request(&mut reader)
        .context("parsing protocol system from request")?;
    let headers = parse_headers_from_request(&mut reader)
        .context("parsing headers from request")?;

    let body = if matches!(method, Method::Post) {
        Some(parse_body_from_request(&mut reader, &headers)
            .context("parsing body from request")?)
    } else {
        None
    };

    Ok(Request {
        method,
        path,
        protocol,
        headers,
        body,
    })
}

fn parse_method_from_request(request: &mut Reader<&[u8]>) -> Result<Method> {
    const SPACE: u8 = b' ';
    let mut method = vec![];

    request.read_until(SPACE, &mut method).context("reading method bytes")?;
    Method::try_from(method)
}

fn parse_path_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    const SPACE: u8 = b' ';
    let mut path_bytes = vec![];

    request.read_until(SPACE, &mut path_bytes).context("reading path bytes")?;
    Ok(String::from_utf8(path_bytes)
        .context("converting path bytes to string")?
        .trim()
        .to_owned())
}

fn parse_protocol_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    let mut protocol = String::new();
    request.read_line(&mut protocol).context("reading protocol line")?;
    Ok(protocol.trim().to_owned())
}

pub type Headers = HashMap<String, String>;

fn parse_headers_from_request(request: &mut Reader<&[u8]>) -> Result<Headers> {
    let mut headers = HashMap::new();

    loop {
        let mut raw_header = String::new();
        request.read_line(&mut raw_header).context("reading header line")?;
        let header = raw_header.trim();

        if header.is_empty() {
            break;
        }

        let mut header_parts = raw_header.splitn(2, ':');
        let header_name = header_parts
            .next()
            .map(|h| h.trim().to_lowercase());
        let header_value = header_parts
            .next()
            .map(|h| h.trim().to_owned());

        if let (Some(name), Some(value)) = (header_name, header_value) {
            headers.insert(name, value);
        }
    }

    Ok(headers)
}

fn parse_body_from_request(request: &mut Reader<&[u8]>, headers: &Headers) -> Result<String> {
    let content_length = headers
        .get("content-length")
        .ok_or_else(|| anyhow::anyhow!("missing content-length header"))?;

    let length = content_length
        .parse::<usize>()
        .context("parsing content-length header into number")?;

    let mut buffer = vec![0; length];
    request
        .read_exact(&mut buffer)
        .context("reading exact body bytes")?;

    Ok(String::from_utf8(buffer).context("converting body buffer to string")?)
}
