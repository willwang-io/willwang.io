use jotdown::{Container, Event};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Metadata {
    pub title: String,
    pub date: toml::value::Datetime,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
}

pub fn parse_djot(content: &str) -> (Metadata, String) {
    let events: Vec<Event> = jotdown::Parser::new(&content).collect();

    let mut in_meta = false;
    let mut have_meta = false;
    let mut meta_buf = String::new();

    let body_events: Vec<Event> = events
        .into_iter()
        .filter_map(|ev| {
            if in_meta {
                match ev {
                    Event::End(Container::RawBlock { .. }) => {
                        in_meta = false;
                        have_meta = true;
                    }
                    Event::Str(s) => meta_buf.push_str(&s),
                    Event::Softbreak | Event::Hardbreak => meta_buf.push('\n'),
                    _ => {}
                }
                None
            } else {
                match ev {
                    Event::Start(Container::RawBlock { ref format }, _)
                        if format.as_ref() == "toml" && !have_meta =>
                    {
                        in_meta = true;
                        None
                    }
                    other => Some(other),
                }
            }
        })
        .collect();

    let metadata = if have_meta {
        match toml::from_str::<Metadata>(&meta_buf) {
            Ok(m) => m,
            Err(err) => {
                panic!("Metadata parse failed: {err}\ninput:\n{meta_buf}");
            }
        }
    } else {
        panic!("All pages must have a metadata section.")
    };

    let html = jotdown::html::render_to_string(body_events.into_iter());

    (metadata, html)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn with_metadata() {
        let content = concat!(
            "```=toml\n",
            "title = \"this is a title\"\n",
            "date = 2025-11-16\n",
            "keywords = [\"foo\", \"bar\"]\n",
            "description = \"Lorem ipsum dolor sit amet\"\n",
            "```\n",
            "\n",
            "hello world\n",
        );
        let (metadata, html) = parse_djot(content);
        assert_eq!(
            Metadata {
                title: String::from("this is a title"),
                date: toml::value::Datetime::from_str("2025-11-16").unwrap(),
                keywords: Some(vec![String::from("foo"), String::from("bar")]),
                description: Some(String::from("Lorem ipsum dolor sit amet")),
            },
            metadata
        );
        assert_eq!("\n<p>hello world</p>\n", &html)
    }

    #[test]
    #[should_panic]
    fn without_metadata() {
        let content = "hello world";
        let _ = parse_djot(content);
    }

    #[test]
    fn only_the_first_raw_toml_will_be_the_metadata() {
        let content = concat!(
            "```=toml\n",
            "title = \"this is a title\"\n",
            "date = 2025-11-16\n",
            "```\n",
            "\n",
            "hello world\n",
            "```=toml\n",
            "foo = \"bar\"\n",
            "```\n",
        );
        let (metadata, html) = parse_djot(content);
        assert_eq!(
            Metadata {
                title: String::from("this is a title"),
                date: toml::value::Datetime::from_str("2025-11-16").unwrap(),
                keywords: None,
                description: None,
            },
            metadata
        );
    }
}
