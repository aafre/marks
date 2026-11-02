use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;

const FILE: &str = "marks.txt";
const USAGE: &str = "usage: marks add <url> [--title <title>] [--tags <a,b>] | marks list";

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

    fn to_line(&self) -> String {
        format!("{}\t{}\t{}", self.url, self.title, self.tags.join(","))
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
            let mut title = String::new();
            let mut tags = Vec::new();
            let mut i = 3;
            while i < args.len() {
                match args[i].as_str() {
                    "--title" => title = args[i + 1].clone(),
                    "--tags" => {
                        for tag in args[i + 1].split(',') {
                            tags.push(tag.to_string());
                        }
                    }
                    _ => {
                        println!("{USAGE}");
                        return Ok(());
                    }
                }
                i += 2;
            }
            let bookmark = Bookmark { url, title, tags };
            add(&bookmark)?;
            println!("Saved {}", bookmark.url);
        }
        "list" => list()?,
        _ => println!("{USAGE}"),
    }

    Ok(())
}

fn add(bookmark: &Bookmark) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).create(true).open(FILE)?;
    writeln!(file, "{}", bookmark.to_line())?;
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
