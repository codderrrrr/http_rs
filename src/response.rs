use std::{ fmt::Display, io::Write, net::TcpStream };

use anyhow::Result;
use anyhow::{ Context, Ok };

use crate::request::Request;

pub fn send_response(response: Response, stream: &mut TcpStream, request: &Request) -> Result<()> {
    write!(stream, "HTTP/1.1 ").context("writing protocol")?;
    write!(stream, "{}\r\n", response.code).context("writing http code")?;

    if response.body.is_some() {
        write!(
            stream,
            "{}{}",
            response.content_type_header(),
            response.content_length_header().unwrap()
        ).context("writing header")?;
    }

    let accept_encoding = request.headers.get("accept-encoding");

    let gzip_encoding = is_gzip_encoding_requested(accept_encoding);

    if gzip_encoding {
        write!(stream, "Content-Encoding: gzip\r\n").context("writing body")?;
    }

    write!(stream, "\r\n").context("writing crlf header")?;

    if let Some(body) = &response.body {
        write!(stream, "{body}").context("writing body")?;
    }

    stream.flush().context("flushing write")?;
    Ok(())
}

#[derive(Debug)]
pub enum HttpCode {
    Ok,
    NotFound,
    YourFault,
    Created,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (number, message) = match self {
            Self::Ok => (200, "Ok"),
            Self::NotFound => (404, "Not Found"),
            Self::YourFault => (400, "Bad Request"),
            Self::Created => (201, "Created"),
        };

        write!(f, "{number} {message}")
    }
}

#[derive(Debug)]
pub struct Response {
    pub code: HttpCode,
    pub body: Option<String>,
    pub content_type: ContentType,
}

impl Response {
    fn content_type_header(&self) -> String {
        let raw_header = match self.content_type {
            ContentType::TextPlain => "text/plain",
            ContentType::ApplicationOctetStream => "application/octet-stream",
        };

        format!("Content-Type: {raw_header}\r\n")
    }

    fn content_length_header(&self) -> Option<String> {
        let Some(body) = self.body.as_ref() else {
            return None;
        };

        let length = body.as_bytes().len();

        Some(format!("Content-Length: {length}\r\n"))
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
}

fn is_gzip_encoding_requested(requested_encodings: Option<&String>) -> bool {
    requested_encodings
        .map(|encoding| {
            let requested_encodings = encoding
                .split(',')
                .map(|encoding: &str| encoding.trim().to_lowercase())
                .collect::<Vec<String>>();

            requested_encodings.iter().any(|encoding: &String| encoding == "gzip")
        })
        .unwrap_or(false)
}
