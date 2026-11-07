fn main() {
    let title = "  The Rust Book ";
    let title = title.trim();
    let mut starred = false;
    println!("{title}, starred: {starred}");
    starred = true;
    println!("{title}, starred: {starred}");
}
