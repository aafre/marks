use std::fmt;

pub struct Bookmark {
    title: Option<String>,
    url: String,
    pub tags: String,
    pub starred: bool,
    kind: Kind,
}

#[derive(Debug, PartialEq)]
enum Kind {
    Video,
    Repo,
    Page,
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let title = match &self.title {
            Some(t) => t.as_str(),
            None => "(untitled)",
        };
        write!(f, "{title} [{:?}] {}", self.kind, self.url)
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

pub fn parse(line: &str) -> Result<Bookmark, String> {
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

#[test]
fn parses_a_line() {
    let b = parse("*Rustlings,https://github.com/rust-lang/rustlings,rust").unwrap();
    assert!(b.starred);
    assert_eq!(b.kind, Kind::Repo);
    assert!(parse("no commas here").is_err());
    assert!(parse("Title,https://example.com").is_err());
}
