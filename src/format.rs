use super::highlight::CodeHighlighter;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Parser, Tag};

pub fn format_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let parser = highlight_code(parser);
    let mut output = String::new();

    html::push_html(&mut output, parser);

    output
}

fn highlight_code<'a, I>(iter: I) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let highlighter = CodeHighlighter::new();
    let events: Vec<Event> = iter.collect();
    let len = events.len();
    let mut new_events: Vec<Event> = Vec::new();
    let mut started_block: Option<CowStr> = None;
    let mut code: Option<CowStr> = None;

    for index in 0..len {
        let event = events[index].clone();

        match event {
            Event::Start(Tag::CodeBlock(block)) => match block {
                CodeBlockKind::Fenced(lang) => started_block = Some(lang),
                CodeBlockKind::Indented => {
                    new_events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)))
                }
            },
            Event::Text(text) => {
                if started_block.is_some() {
                    code = Some(text)
                } else {
                    new_events.push(Event::Text(text))
                }
            }
            Event::End(Tag::CodeBlock(block)) => match block {
                CodeBlockKind::Fenced(_lang) => {
                    let html = highlighter.highligt_code(
                        &started_block.clone().expect("No lang"),
                        &code.clone().expect("No Code"),
                    );

                    if html.is_some() {
                        new_events.push(Event::Html(html.expect("no formatted code").into()))
                    }
                }
                CodeBlockKind::Indented => {
                    new_events.push(Event::End(Tag::CodeBlock(CodeBlockKind::Indented)))
                }
            },
            other => new_events.push(other),
        }
    }

    new_events.into_iter()
}
