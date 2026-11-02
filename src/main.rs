use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

const FILE: &str = "marks.txt";
const USAGE: &str = "usage: marks add <url> [--title <title>] [--tags <a,b>] | marks list | marks version";

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
    Version,
    Help,
}

enum MarksError {
    Io(io::Error),
    UnknownCommand(String),
    MissingUrl,
    MissingValue(String),
    UnknownFlag(String),
}

impl fmt::Display for MarksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarksError::Io(err) => write!(f, "couldn't use {FILE}: {err}"),
            MarksError::UnknownCommand(name) => write!(f, "unknown command '{name}'"),
            MarksError::MissingUrl => write!(f, "add needs a URL"),
            MarksError::MissingValue(flag) => write!(f, "{flag} needs a value"),
            MarksError::UnknownFlag(flag) => write!(f, "unknown flag '{flag}'"),
        }
    }
}

impl From<io::Error> for MarksError {
    fn from(err: io::Error) -> MarksError {
        MarksError::Io(err)
    }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(err) = run(&args) {
        eprintln!("error: {err}");
        eprintln!("{USAGE}");
        process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), MarksError> {
    match parse(args)? {
        Command::Add { url, title, tags } => add(Bookmark { url, title, tags })?,
        Command::List => list()?,
        Command::Version => println!("marks {}", env!("CARGO_PKG_VERSION")),
        Command::Help => println!("{USAGE}"),
    }
    Ok(())
}

fn parse(args: &[String]) -> Result<Command, MarksError> {
    let name = match args.get(1) {
        Some(name) => name,
        None => return Ok(Command::Help),
    };
    match name.as_str() {
        "add" => parse_add(&args[2..]),
        "list" => Ok(Command::List),
        "version" => Ok(Command::Version),
        "help" => Ok(Command::Help),
        other => Err(MarksError::UnknownCommand(other.to_string())),
    }
}

fn parse_add(args: &[String]) -> Result<Command, MarksError> {
    let url = match args.first() {
        Some(url) => url.clone(),
        None => return Err(MarksError::MissingUrl),
    };
    let mut title = String::new();
    let mut tags = Vec::new();
    let mut i = 1;
    while i < args.len() {
        let flag = &args[i];
        let value = match args.get(i + 1) {
            Some(value) => value,
            None => return Err(MarksError::MissingValue(flag.clone())),
        };
        match flag.as_str() {
            "--title" => title = value.clone(),
            "--tags" => {
                for tag in value.split(',') {
                    tags.push(tag.to_string());
                }
            }
            _ => return Err(MarksError::UnknownFlag(flag.clone())),
        }
        i += 2;
    }
    Ok(Command::Add { url, title, tags })
}

fn add(bookmark: Bookmark) -> Result<(), MarksError> {
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

fn save(bookmarks: &Vec<Bookmark>) -> Result<(), MarksError> {
    let mut contents = String::new();
    for bookmark in bookmarks {
        contents.push_str(&bookmark.to_line());
        contents.push('\n');
    }
    fs::write(FILE, contents)?;
    Ok(())
}

fn load() -> Result<Vec<Bookmark>, MarksError> {
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

fn list() -> Result<(), MarksError> {
    let bookmarks = load()?;
    println!("{:>3}  {:<24} {:<36} {}", "#", "TITLE", "URL", "TAGS");
    for (i, bookmark) in bookmarks.iter().enumerate() {
        println!("{:>3}  {}", i + 1, bookmark.row());
    }
    Ok(())
}
