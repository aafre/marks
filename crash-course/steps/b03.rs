fn main() {
    let title = "  The Rust Book ";
    let title = title.trim();
    let mut starred = false;
    println!("{title}, starred: {starred}");
    starred = true;

    let link = (title, "https://doc.rust-lang.org/book");
    println!("{} -> {}, starred: {starred}", link.0, link.1);
}
