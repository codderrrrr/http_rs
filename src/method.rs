use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub enum Method {
    Get = 0,
}

impl TryFrom<Vec<u8>> for Method{
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        let method_string = String::from_utf8(value).context("Converting bytes to method string")?;

        Ok(match method_string.to_uppercase().trim() {
            "GET" => Self::Get,
            _ => bail!("Unknown method"),
        })
    }
}