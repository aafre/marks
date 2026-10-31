use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;

const FILE: &str = "marks.txt";
const USAGE: &str = "usage: marks add <url> | marks list";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{USAGE}");
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            let url = args[2].clone();
            let mut file = OpenOptions::new().append(true).create(true).open(FILE)?;
            writeln!(file, "{url}")?;
            println!("Saved {url}");
        }
        "list" => list()?,
        _ => println!("{USAGE}"),
    }

    Ok(())
}

fn list() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(FILE)?;
    for (i, line) in contents.lines().enumerate() {
        println!("{}. {line}", i + 1);
    }
    Ok(())
}
