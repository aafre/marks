mod bookmark;

use bookmark::{Bookmark, parse};
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let text = fs::read_to_string("bookmarks.txt")?;
    let mut bookmarks = Vec::new();
    for line in text.lines() {
        match parse(line) {
            Ok(b) => bookmarks.push(b),
            Err(e) => eprintln!("warning: skipping {line:?}: {e}"),
        }
    }

    let mut by_tag: HashMap<&str, Vec<&Bookmark>> = HashMap::new();
    for b in &bookmarks {
        for tag in b.tags.split_whitespace() {
            by_tag.entry(tag).or_default().push(b);
        }
    }

    println!("{} bookmarks, {} tags", bookmarks.len(), by_tag.len());
    println!("#rust has {} bookmarks", by_tag["rust"].len());

    let starred: Vec<&Bookmark> = bookmarks.iter().filter(|b| b.starred).collect();
    println!("starred:");
    for b in &starred {
        println!("  {b}");
    }
    Ok(())
}
