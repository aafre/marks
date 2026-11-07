use std::collections::HashMap;
use std::fs;

struct Bookmark {
    title: Option<String>,
    url: String,
    tags: String,
    starred: bool,
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

fn parse(line: &str) -> Result<Bookmark, String> {
    let starred = line.starts_with('*');
    let line = line.trim_start_matches('*');
    let (title, rest) = line.split_once(',').ok_or("expected title,url,tags")?;
    let (url, tags) = rest.split_once(',').ok_or("missing tags")?;
    let title = match title {
        "" => None,
        t => Some(t.to_string()),
    };
    Ok(Bookmark {
        title,
        url: url.to_string(),
        tags: tags.to_string(),
        starred,
        kind: kind(url),
    })
}

fn main() -> Result<(), std::io::Error> {
    let text = fs::read_to_string("bookmarks.txt")?;
    let mut bookmarks = Vec::new();
    for line in text.lines() {
        match parse(line) {
            Ok(b) => bookmarks.push(b),
            Err(e) => eprintln!("warning: skipping {line:?}: {e}"),
        }
    }

    let mut by_tag: HashMap<&str, Vec<&Bookmark>> = HashMap::new();
    for b in &bookmarks {
        for tag in b.tags.split_whitespace() {
            by_tag.entry(tag).or_default().push(b);
        }
    }

    println!("{} bookmarks, {} tags", bookmarks.len(), by_tag.len());
    println!("#rust has {} bookmarks", by_tag["rust"].len());
    for b in &bookmarks {
        if b.starred {
            b.show();
        }
    }
    Ok(())
}
