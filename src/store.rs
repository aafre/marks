use std::fs;
use std::path::Path;

use crate::bookmark::Bookmark;
use crate::error::MarksError;

pub trait Store {
    fn load(&self) -> Result<Vec<Bookmark>, MarksError>;
    fn save(&mut self, bookmarks: &Vec<Bookmark>) -> Result<(), MarksError>;
}

pub struct TextStore {
    pub path: String,
}

impl Store for TextStore {
    fn load(&self) -> Result<Vec<Bookmark>, MarksError> {
        let mut bookmarks = Vec::new();
        if !Path::new(&self.path).exists() {
            return Ok(bookmarks);
        }
        let contents = fs::read_to_string(&self.path)?;
        for line in contents.lines() {
            if let Some(bookmark) = Bookmark::from_line(line) {
                bookmarks.push(bookmark);
            }
        }
        Ok(bookmarks)
    }

    fn save(&mut self, bookmarks: &Vec<Bookmark>) -> Result<(), MarksError> {
        let mut contents = String::new();
        for bookmark in bookmarks {
            contents.push_str(&bookmark.to_line());
            contents.push('\n');
        }
        fs::write(&self.path, contents)?;
        Ok(())
    }
}
