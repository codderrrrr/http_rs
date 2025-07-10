use std::{fs::File, io::{ErrorKind, Read}};
use anyhow::{Context, Result};
use crate::response::{Response, HttpCode, ContentType};

pub fn handle_file(file_name: Option<&str>, directory: Option<String>) -> Result<Response> {
    let Some(directory) = directory else {
        return Ok(Response {
            code: HttpCode::YourFault,
            body: None,
            content_type: ContentType::TextPlain,
            gzip_encoding: false,
            encoded_body: None,
        });
    };

    let Some(file_name) = file_name else {
        return Ok(Response {
            code: HttpCode::NotFound,
            body: None,
            content_type: ContentType::TextPlain,
            gzip_encoding: false,
            encoded_body: None,
        });
    };

    let path = format!("{}/{}", directory, file_name);

    // Open the file, handle NotFound specifically
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            return Ok(Response {
                code: HttpCode::NotFound,
                body: None,
                content_type: ContentType::TextPlain,
                gzip_encoding: false,
                encoded_body: None,
            });
        }
        Err(error) => return Err(error).context("opening file"),
    };

    // Read contents
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("reading file to string")?;

    Ok(Response {
        code: HttpCode::Ok,
        body: Some(contents),
        content_type: ContentType::ApplicationOctetStream,
        gzip_encoding: false,
        encoded_body: None,
    })
}
