use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MarksError {
    Io(io::Error),
    Json(serde_json::Error),
    NoSuchBookmark(usize),
}

impl fmt::Display for MarksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarksError::Io(err) => write!(f, "couldn't use the bookmarks file: {err}"),
            MarksError::Json(err) => write!(f, "the bookmarks file isn't valid JSON: {err}"),
            MarksError::NoSuchBookmark(number) => write!(f, "there's no bookmark #{number}"),
        }
    }
}

impl From<io::Error> for MarksError {
    fn from(err: io::Error) -> MarksError {
        MarksError::Io(err)
    }
}

impl From<serde_json::Error> for MarksError {
    fn from(err: serde_json::Error) -> MarksError {
        MarksError::Json(err)
    }
}
