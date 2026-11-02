use clap::{Parser, Subcommand};
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

const FILE: &str = "marks.txt";

struct Bookmark {
    url: String,
    title: String,
    tags: Vec<String>,
}

/// A bookmark manager for the terminal
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Save a link
    Add {
        url: String,
        /// A title to show in the list
        #[arg(long)]
        title: Option<String>,
        /// Tags, separated by commas
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    /// Show every saved link
    List,
}

enum MarksError {
    Io(io::Error),
}

impl fmt::Display for MarksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarksError::Io(err) => write!(f, "couldn't use {FILE}: {err}"),
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
    let cli = Cli::parse();
    if let Err(err) = run(cli.command) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run(command: Command) -> Result<(), MarksError> {
    match command {
        Command::Add { url, title, tags } => {
            let title = title.unwrap_or_default();
            add(Bookmark { url, title, tags })?
        }
        Command::List => list()?,
    }
    Ok(())
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
