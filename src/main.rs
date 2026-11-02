use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;

const FILE: &str = "marks.txt";
const USAGE: &str = "usage: marks add <url> | marks list";

struct Bookmark {
    url: String,
    title: String,
    tags: Vec<String>,
}

impl Bookmark {
    fn from_line(line: &str) -> Option<Bookmark> {
        if line.trim().is_empty() {
            return None;
        }
        let mut fields = line.split('\t');
        let url = fields.next().unwrap_or("").to_string();
        let title = fields.next().unwrap_or("").to_string();
        let mut tags = Vec::new();
        for tag in fields.next().unwrap_or("").split(',') {
            if !tag.is_empty() {
                tags.push(tag.to_string());
            }
        }
        Some(Bookmark { url, title, tags })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{USAGE}");
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            let url = args[2].clone();
            add(&url)?;
            println!("Saved {url}");
        }
        "list" => list()?,
        _ => println!("{USAGE}"),
    }

    Ok(())
}

fn add(url: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).create(true).open(FILE)?;
    writeln!(file, "{url}")?;
    Ok(())
}

fn list() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(FILE)?;
    for (i, line) in contents.lines().enumerate() {
        if let Some(bookmark) = Bookmark::from_line(line) {
            println!("{}. {} {} {:?}", i + 1, bookmark.url, bookmark.title, bookmark.tags);
        }
    }
    Ok(())
}
