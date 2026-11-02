#[derive(Clone)]
pub struct Bookmark {
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    pub fn from_line(line: &str) -> Option<Bookmark> {
        if line.trim().is_empty() {
            return None;
        }
        let mut fields = line.split('\t');
        let url = fields.next().unwrap_or("").to_string();
        let title = fields.next().unwrap_or("").to_string();
        let mut tags = Vec::new();
        for tag in fields.next().unwrap_or("").split(',') {
            if !tag.is_empty() {
                tags.push(tag.to_string());
            }
        }
        Some(Bookmark { url, title, tags })
    }

    pub fn to_line(&self) -> String {
        format!("{}\t{}\t{}", self.url, self.title, self.tags.join(","))
    }

    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.title.to_lowercase().contains(&query)
            || self.url.contains(&query)
            || self.tags.iter().any(|tag| *tag == query)
    }

    pub fn row(&self) -> String {
        let title = if self.title.is_empty() { "(untitled)" } else { &self.title };
        format!("{:<24} {:<36} {}", title, self.url, self.tags.join(", "))
    }
}
