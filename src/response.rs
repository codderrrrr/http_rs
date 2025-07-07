use std::{ fmt::Display, io::Write, net::TcpStream};

use anyhow::Result;
use anyhow::{ Context, Ok };

pub fn send_response(response: Response, stream: &mut TcpStream) -> Result<()> {
    write!(stream, "HTTP/1.1 ").context("writing protocol")?;
    write!(stream, "{}\r\n", response.code).context("writing http code")?;

    if response.body.is_some() {
        write!(stream, "{}{}", response.content_type_header(), response.content_length_header().unwrap()).context("writing header")?;
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

#[derive(Debug)]
pub struct Response {
    pub code: HttpCode,
    pub body: Option<String>,
}

impl Response {
    fn content_type_header(&self) ->String {
        "Content-Type: text/plain\r\n".to_owned()
    }

    fn content_length_header(&self) -> Option<String> {
        let Some(body) = self.body.as_ref() else{
            return None;
        };

        let length = body.as_bytes().len();

        Some(format!("Content-Length: {length}\r\n"))
    }
}
