use std::{fs::File, io::Write, path::Path};

use anyhow::{Context, Result};
use crate::{request::Request, response::{Response, HttpCode, ContentType}};

pub fn save_file(
    directory: Option<String>,
    file_name: Option<&str>,
    request: &Request
) -> Result<Response> {
    // Check for directory
    let Some(directory) = directory else {
        return Ok(Response {
            code: HttpCode::YourFault,
            body: None,
            content_type: ContentType::TextPlain,
            gzip_encoding: false,
            encoded_body: None,
        });
    };

    // Check for file name
    let Some(file_name) = file_name else {
        return Ok(Response {
            code: HttpCode::YourFault,
            body: None,
            content_type: ContentType::TextPlain,
            gzip_encoding: false,
            encoded_body: None,
        });
    };

    // Check for body in request
    let Some(body) = request.body.as_ref() else {
        return Ok(Response {
            code: HttpCode::YourFault,
            body: None,
            content_type: ContentType::TextPlain,
            gzip_encoding: false,
            encoded_body: None,
        });
    };

    // Create the file
    let path = Path::new(&directory).join(file_name);
    let mut file = File::create(&path).context("opening file for write")?;

    file.write_all(body.as_bytes()).context("writing body to file")?;
    file.flush().context("flushing body")?;

    Ok(Response {
        code: HttpCode::Created,
        body: None,
        content_type: ContentType::TextPlain,
        gzip_encoding: false,
        encoded_body: None,
    })
}

