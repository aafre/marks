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

fn main() -> Result<(), Box<dyn Error>> {
    let bookmark = Bookmark {
        url: String::from("https://doc.rust-lang.org/book/"),
        title: String::from("The Rust Book"),
        tags: vec![String::from("rust"), String::from("docs")],
    };
    println!("{} -> {}", bookmark.title, bookmark.url);

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
        println!("{}. {line}", i + 1);
    }
    Ok(())
}
