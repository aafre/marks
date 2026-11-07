struct Bookmark {
    title: String,
    url: String,
}

impl Bookmark {
    fn show(&self) {
        println!("{} ({})", self.title, domain(&self.url));
    }
}

fn domain(url: &str) -> &str {
    url.split('/').nth(2).unwrap_or(url)
}

fn main() {
    let b = Bookmark {
        title: String::from("The Rust Book"),
        url: String::from("https://doc.rust-lang.org/book"),
    };
    b.show();
}
