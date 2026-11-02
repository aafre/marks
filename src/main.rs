use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

const FILE: &str = "marks.txt";
const USAGE: &str = "usage: marks add <url> [--title <title>] [--tags <a,b>] | marks list";

struct Bookmark {
    url: String,
    title: String,
    tags: Vec<String>,
}

enum Command {
    Add {
        url: String,
        title: String,
        tags: Vec<String>,
    },
    List,
    Help,
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

    fn row(&self) -> String {
        let title = if self.title.is_empty() { "(untitled)" } else { &self.title };
        format!("{:<24} {:<36} {}", title, self.url, self.tags.join(", "))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match parse(&args) {
        Command::Add { url, title, tags } => add(Bookmark { url, title, tags })?,
        Command::List => list()?,
        Command::Help => println!("{USAGE}"),
    }

    Ok(())
}

fn parse(args: &[String]) -> Command {
    let name = match args.get(1) {
        Some(name) => name,
        None => return Command::Help,
    };
    match name.as_str() {
        "add" => parse_add(&args[2..]),
        "list" => Command::List,
        _ => Command::Help,
    }
}

fn parse_add(args: &[String]) -> Command {
    let url = match args.first() {
        Some(url) => url.clone(),
        None => return Command::Help,
    };
    let mut title = String::new();
    let mut tags = Vec::new();
    let mut i = 1;
    while i < args.len() {
        let value = match args.get(i + 1) {
            Some(value) => value,
            None => return Command::Help,
        };
        match args[i].as_str() {
            "--title" => title = value.clone(),
            "--tags" => {
                for tag in value.split(',') {
                    tags.push(tag.to_string());
                }
            }
            _ => return Command::Help,
        }
        i += 2;
    }
    Command::Add { url, title, tags }
}

fn add(bookmark: Bookmark) -> Result<(), Box<dyn Error>> {
    let mut bookmarks = load()?;
    let mut existing = None;
    for saved in &bookmarks {
        if saved.url == bookmark.url {
            existing = Some(saved);
        }
    }
    if let Some(saved) = existing {
        println!("Already saved:  {}", saved.row());
    }
    println!("Saved {}", bookmark.url);
    bookmarks.push(bookmark);
    save(&bookmarks)
}

fn save(bookmarks: &Vec<Bookmark>) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    for bookmark in bookmarks {
        contents.push_str(&bookmark.to_line());
        contents.push('\n');
    }
    fs::write(FILE, contents)?;
    Ok(())
}

fn load() -> Result<Vec<Bookmark>, Box<dyn Error>> {
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

fn list() -> Result<(), Box<dyn Error>> {
    let bookmarks = load()?;
    println!("{:>3}  {:<24} {:<36} {}", "#", "TITLE", "URL", "TAGS");
    for (i, bookmark) in bookmarks.iter().enumerate() {
        println!("{:>3}  {}", i + 1, bookmark.row());
    }
    Ok(())
}
