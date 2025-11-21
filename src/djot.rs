use jotdown::{Container, Event};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Metadata {
    title: String,
    date: toml::value::Datetime,
    draft: bool,
}

fn parse_djot(content: &str) -> (Option<Metadata>, String) {
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
        toml::from_str::<Metadata>(&meta_buf).ok()
    } else {
        None
    };

    let html = jotdown::html::render_to_string(body_events.into_iter());

    (metadata, html)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn djot_with_metadata() {
        let content = concat!(
            "```=toml\n",
            "title = \"this is a title\"\n",
            "date = 2025-11-16\n",
            "draft = true\n",
            "```\n",
            "\n",
            "hello world\n",
        );
        let (metadata, html) = parse_djot(content);
        assert_eq!(
            Some(Metadata {
                title: String::from("this is a title"),
                date: toml::value::Datetime::from_str("2025-11-16").unwrap(),
                draft: true,
            }),
            metadata
        );
        assert_eq!("\n<p>hello world</p>\n", &html)
    }

    #[test]
    fn djot_without_metadata() {
        let content = "hello world";
        let (metadata, html) = parse_djot(content);
        assert_eq!(None, metadata);
        assert_eq!("<p>hello world</p>\n", &html)
    }
}
