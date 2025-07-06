use anyhow::{ bail, Context, Result };
use crate::{ request::Request, response::{ HttpCode, Response } };
mod home;
mod echo;

pub fn router(request: Request) -> Result<Response> {
    let mut segments = request.path.trim_matches('/').split('/');

    let res = match segments.next() {
        Some("") => home::home().context("home request processing")?,
        Some("echo") => echo::echo(segments.next()).context("processing echo handler")?,
        Some(_) => Response { code: HttpCode::NotFound, body: None },
        None => bail!("Did not get any segments"),
    };

    Ok(res)
}

