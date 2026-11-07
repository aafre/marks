struct Bookmark {
    title: Option<String>,
    url: String,
    kind: Kind,
}

enum Kind {
    Video,
    Repo,
    Page,
}

impl Bookmark {
    fn show(&self) {
        let title = match &self.title {
            Some(t) => t.as_str(),
            None => "(untitled)",
        };
        let kind = match self.kind {
            Kind::Video => "video",
            Kind::Repo => "repo",
            Kind::Page => "page",
        };
        println!("{title} [{kind}] {}", self.url);
    }
}

fn domain(url: &str) -> &str {
    url.split('/').nth(2).unwrap_or(url)
}

fn kind(url: &str) -> Kind {
    match domain(url) {
        "www.youtube.com" => Kind::Video,
        "github.com" => Kind::Repo,
        _ => Kind::Page,
    }
}

fn main() {
    let url = String::from("https://doc.rust-lang.org/book");
    let book = Bookmark {
        title: Some(String::from("The Rust Book")),
        kind: kind(&url),
        url,
    };
    let url = String::from("https://www.youtube.com/@RustVideos");
    let video = Bookmark {
        title: None,
        kind: kind(&url),
        url,
    };
    book.show();
    video.show();
}
