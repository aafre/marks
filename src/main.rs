mod bookmark;
mod error;
mod store;

use clap::{Parser, Subcommand};
use std::process;

use bookmark::Bookmark;
use error::MarksError;
use store::{JsonStore, Store, TextStore};

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
    let mut store = JsonStore {
        path: String::from("marks.json"),
    };
    if let Err(err) = run(&mut store, cli.command) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run<S: Store>(store: &mut S, command: Command) -> Result<(), MarksError> {
    import_text_file(store)?;
    match command {
        Command::Add { url, title, tags } => {
            let title = title.unwrap_or_default();
            add(store, Bookmark { url, title, tags })?
        }
        Command::List => list(store)?,
        Command::Search { query } => {
            let bookmarks = store.load()?;
            for bookmark in search(&bookmarks, &query) {
                println!("{}", bookmark.row());
            }
        }
        Command::Tag { number, tag } => add_tag(store, number, tag)?,
    }
    Ok(())
}

fn add<S: Store>(store: &mut S, bookmark: Bookmark) -> Result<(), MarksError> {
    let mut bookmarks = store.load()?;
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
    store.save(&bookmarks)
}

fn add_tag<S: Store>(store: &mut S, number: usize, tag: String) -> Result<(), MarksError> {
    let mut bookmarks = store.load()?;
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
    store.save(&bookmarks)
}

fn search(bookmarks: &[Bookmark], query: &str) -> Vec<Bookmark> {
    bookmarks.iter().filter(|bookmark| bookmark.matches(query)).cloned().collect()
}

fn list<S: Store>(store: &S) -> Result<(), MarksError> {
    let bookmarks = store.load()?;
    println!("{:>3}  {:<24} {:<36} {}", "#", "TITLE", "URL", "TAGS");
    for (i, bookmark) in bookmarks.iter().enumerate() {
        println!("{:>3}  {}", i + 1, bookmark.row());
    }
    Ok(())
}

fn import_text_file<S: Store>(store: &mut S) -> Result<(), MarksError> {
    let text_store = TextStore {
        path: String::from("marks.txt"),
    };
    let old = text_store.load()?;
    if old.is_empty() || !store.load()?.is_empty() {
        return Ok(());
    }
    store.save(&old)?;
    println!("Imported {} bookmarks from marks.txt", old.len());
    Ok(())
}
