use std::fs;
use std::path::Path;

use crate::bookmark::Bookmark;
use crate::error::MarksError;

const FILE: &str = "marks.txt";

pub fn save(bookmarks: &Vec<Bookmark>) -> Result<(), MarksError> {
    let mut contents = String::new();
    for bookmark in bookmarks {
        contents.push_str(&bookmark.to_line());
        contents.push('\n');
    }
    fs::write(FILE, contents)?;
    Ok(())
}

pub fn load() -> Result<Vec<Bookmark>, MarksError> {
    let mut bookmarks = Vec::new();
    if !Path::new(FILE).exists() {
        return Ok(bookmarks);
    }
    let contents = fs::read_to_string(FILE)?;
    for line in contents.lines() {
        if let Some(bookmark) = Bookmark::from_line(line) {
            bookmarks.push(bookmark);
        }
    }
    Ok(bookmarks)
}
