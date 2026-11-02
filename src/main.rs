mod bookmark;
mod error;
mod store;

use clap::{Parser, Subcommand};
use std::process;

use bookmark::Bookmark;
use error::MarksError;
use store::{load, save};

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
    /// Find links by title, URL or tag
    Search { query: String },
    /// Add a tag to a saved link
    Tag { number: usize, tag: String },
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
        Command::Search { query } => {
            let bookmarks = load()?;
            for bookmark in search(&bookmarks, &query) {
                println!("{}", bookmark.row());
            }
        }
        Command::Tag { number, tag } => add_tag(number, tag)?,
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

fn add_tag(number: usize, tag: String) -> Result<(), MarksError> {
    let mut bookmarks = load()?;
    let bookmark = match number.checked_sub(1).and_then(|index| bookmarks.get_mut(index)) {
        Some(bookmark) => bookmark,
        None => return Err(MarksError::NoSuchBookmark(number)),
    };
    if bookmark.tags.iter().any(|existing| *existing == tag) {
        println!("#{number} is already tagged {tag}");
        return Ok(());
    }
    bookmark.tags.push(tag);
    println!("Tagged:  {}", bookmark.row());
    save(&bookmarks)
}

fn search(bookmarks: &[Bookmark], query: &str) -> Vec<Bookmark> {
    bookmarks.iter().filter(|bookmark| bookmark.matches(query)).cloned().collect()
}

fn list() -> Result<(), MarksError> {
    let bookmarks = load()?;
    println!("{:>3}  {:<24} {:<36} {}", "#", "TITLE", "URL", "TAGS");
    for (i, bookmark) in bookmarks.iter().enumerate() {
        println!("{:>3}  {}", i + 1, bookmark.row());
    }
    Ok(())
}
