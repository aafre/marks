fn marker(starred: bool) -> char {
    if starred { '*' } else { ' ' }
}

fn print_url(url: &str) {
    println!("url: {url}");
}

fn add_tag(tags: &mut String, tag: &str) {
    tags.push(' ');
    tags.push_str(tag);
}

fn main() {
    let title = "  The Rust Book ";
    let title = title.trim();
    let mut starred = false;
    println!("{title}, starred: {starred}");
    starred = true;

    let link = (title, "https://doc.rust-lang.org/book");
    println!("{} {} -> {}", marker(starred), link.0, link.1);
    println!("starred: {starred}");

    let tags = "rust docs";
    for tag in tags.split_whitespace() {
        println!("#{tag}");
    }

    let url = String::from(link.1);
    println!("url: {} bytes", url.len());
    print_url(&url);
    print_url(&url);

    let mut tags = String::from(tags);
    add_tag(&mut tags, "book");
    println!("tags: {tags}");
}
