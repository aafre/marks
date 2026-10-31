use std::env;

const USAGE: &str = "usage: marks add <url> | marks list";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{USAGE}");
        return;
    }

    match args[1].as_str() {
        "add" => println!("adding {}", args[2]),
        "list" => println!("listing bookmarks"),
        _ => println!("{USAGE}"),
    }
}
