use anyhow::{Context, Result};
use bytes::{buf::Reader, Buf};

use crate::method::Method;
use std::io::BufRead;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Request {
    pub method: Method,
    pub path: String,
}

pub fn parse_raw_request(request: Vec<u8>) ->  Result<Request> {
    let mut reader = request.reader();
    let method = parse_method_from_request(&mut reader).context("parsing method")?;
    let path = parse_path_from_request(&mut reader).context("parsing path")?;
    Ok(Request { method, path })

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