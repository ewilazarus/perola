use std::string::ToString;
use chrono::{NaiveDateTime, Utc};

pub struct Perl {
    pub author: String,
    pub content: String,
    pub context: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Perl {
    pub fn new(author: &str, content: &str, context: Option<&str>) -> Perl {
        Perl {
            author: Perl::beautify_author(author),
            content: String::from(content),
            context: match context {
                Some(ctx) => Some(String::from(ctx)),
                None => None,
            },
            created_at: Utc::now().naive_utc(),
        }
    }

    fn beautify_author(author: &str) -> String {
        let segments: Vec<&str> = author.split(" ").collect();
        if segments.len() == 1 {
            return String::from(segments[0]);
        } else {
            return format!("{0}, {1}", segments[segments.len()-1], segments[..segments.len()-1].join(" "))
        }
    }
}

impl ToString for Perl {
    fn to_string(&self) -> String { 
        if let Some(context) = &self.context {
            format!(
                "> [{0}]\n\n  {1}\n\n  -- {2} @ {3}",
                self.created_at,
                self.content,
                self.author,
                context,
            )
        } else {
            format!(
                "> [{0}]\n\n  {1}\n\n  -- {2}",
                self.created_at,
                self.content,
                self.author,
            )
        }
    }
}
