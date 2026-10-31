use std::env;
use std::error::Error;
use std::fs::OpenOptions;
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
        "list" => println!("listing bookmarks"),
        _ => println!("{USAGE}"),
    }

    Ok(())
}
