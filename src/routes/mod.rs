use anyhow::{ bail, Context, Result };
use crate::{ method::Method, request::Request, response::{ HttpCode, Response } };
mod home;
mod echo;
mod user_agent;
mod save_file;
use save_file::save_file;
use user_agent::user_agent;
mod file_handler;
use file_handler::handle_file;

pub fn router(request: &Request, directory: Option<String>) -> Result<Response> {
    let mut segments = request.path.trim_matches('/').split('/');

    let res = match segments.next() {
        Some("") => home::home().context("home request processing")?,
        Some("echo") => echo::echo(segments.next()).context("processing echo handler")?,
        Some("user-agent") => user_agent(&request).context("running user agent handler")?,
        Some("files") => {
            match request.method {
                Method::Get => handle_file(segments.next(), directory).context("handling files")?,
                Method::Post =>
                    save_file(directory, segments.next(), &request).context("saving file")?,
            }
        }
        Some(_) =>
            Response {
                code: HttpCode::NotFound,
                body: None,
                content_type: crate::response::ContentType::TextPlain,
            },
        None => bail!("Did not get any segments"),
    };

    Ok(res)
}
