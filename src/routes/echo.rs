use anyhow::{bail, Result};
use crate::response::Response;

pub fn echo(path_param: Option<&str>) -> Result<Response> {
    let Some(param) = path_param else {
        bail!("missing path param");
    };

    let response = Response {
        code: crate::response::HttpCode::Ok,
        body: Some(param.to_owned()),
        content_type: crate::response::ContentType::TextPlain,
    };

    Ok(response)
}