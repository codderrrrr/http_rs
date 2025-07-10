use anyhow::Result;

use crate::response::Response;

pub fn home() -> Result<Response> {
    Ok(Response { 
        code: crate::response::HttpCode::Ok, 
        body: None,
        content_type: crate::response::ContentType::TextPlain,
        gzip_encoding: false,
        encoded_body: None,
    })
}
