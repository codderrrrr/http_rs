use std::fmt::{Display};

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