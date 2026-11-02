use std::fmt;
use std::io;

pub enum MarksError {
    Io(io::Error),
    NoSuchBookmark(usize),
}

impl fmt::Display for MarksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarksError::Io(err) => write!(f, "couldn't use the bookmarks file: {err}"),
            MarksError::NoSuchBookmark(number) => write!(f, "there's no bookmark #{number}"),
        }
    }
}

impl From<io::Error> for MarksError {
    fn from(err: io::Error) -> MarksError {
        MarksError::Io(err)
    }
}
