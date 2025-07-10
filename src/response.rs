use std::{ fmt::Display, io::Write, net::TcpStream };
use anyhow::{ Result, Context, Ok, anyhow };
use flate2::{ write::GzEncoder, Compression };

use crate::request::Request;

pub fn send_response(
    mut response: Response,
    stream: &mut TcpStream,
    request: &Request
) -> Result<()> {
    response.encode_body(request).context("encoding body")?;

    write!(stream, "HTTP/1.1 {}\r\n", response.code).context("writing status line")?;
    write!(stream, "{}", response.content_type_header()).context("writing content-type")?;

    if let Some(length_header) = response.content_length_header() {
        write!(stream, "{}", length_header).context("writing content-length")?;
    }
    if response.gzip_encoding {
        write!(stream, "Content-Encoding: gzip\r\n").context("writing content-encoding")?;
    }

    write!(stream, "\r\n").context("writing CRLF header")?;
    
    if response.gzip_encoding {
        stream
            .write_all(&response.encoded_body.ok_or(anyhow!("missing encoded body"))?)
            .context("writing gzip encoded body to stream")?;
    } else if let Some(body) = &response.body {
        stream
            .write_all(body.as_bytes())
            .context("writing body")?;
    }

    stream.flush().context("flushing stream")?;
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
    pub gzip_encoding: bool,
    pub encoded_body: Option<Vec<u8>>,
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

        let length = if self.gzip_encoding {
            self.encoded_body.as_ref()?.len()
        } else {
            body.as_bytes().len()
        };

        Some(format!("Content-Length: {length}\r\n"))
    }

    pub fn encode_body(&mut self, request: &Request) -> Result<()> {
        let accept_encoding = request.headers.get("accept-encoding");
        self.gzip_encoding = is_gzip_encoding_requested(accept_encoding);

        let Some(body) = self.body.as_ref() else {
            return Ok(());
        };

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(body.as_bytes()).context("writing compressed body")?;

        self.encoded_body = Some(encoder.finish().context("finishing writing")?);
        Ok(())
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
