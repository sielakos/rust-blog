use crate::format::format_markdown;
use anyhow::{Context, Result};
use pulldown_cmark::{Event, Parser};
use serde::Serialize;
use std::{
    fs::{create_dir_all, read_dir, File},
    io::{Read, Write},
    path::Path,
};
use tera::{Context as TeraContext, Tera};

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    pub content: String,
    pub title: String,
    pub filename: String,
}

impl Post {
    pub fn save(&self, directory: &Path) -> Result<()> {
        let path = directory.join(format!("{}.{}", &self.filename, "html"));
        let mut file = File::create(path)?;

        file.write_all(self.content.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn read(path: &Path, tera: &Tera) -> Result<Post> {
        let filename = path
            .file_stem()
            .context("There is no file name")?
            .to_str()
            .context("Could not convert file name to str")?
            .to_owned();
        let mut file = File::open(path)?;
        let mut input = String::new();

        file.read_to_string(&mut input)?;

        let markdown = format_markdown(&input);
        let title = Post::read_title(&input).context("Could not find title")?;

        let mut context = TeraContext::new();

        context.insert("content", &markdown);
        context.insert("title", &title);

        let content = tera.render("document.html", &context)?;

        Ok(Post {
            title,
            content,
            filename,
        })
    }

    fn read_title(input: &str) -> Option<String> {
        use Event::Text;

        let events = Parser::new(&input)
            .filter(|event| if let Text(_) = event { true } else { false })
            .take(1)
            .collect::<Vec<_>>();

        // It works under assumption that title is first text inside markdown document
        events.get(0).and_then(|event| {
            if let Text(text) = event {
                Some(text.to_string())
            } else {
                None
            }
        })
    }
}

pub fn read_posts<I: AsRef<Path>>(directory: &I, tera: &Tera) -> Result<Vec<Post>> {
    let res = read_dir(directory)?
        .map(move |file| {
            let file = file?;

            Post::read(&file.path(), tera)
        })
        .collect::<Vec<_>>();

    res.into_iter().collect::<Result<Vec<_>>>()
}

pub fn save_posts<I: AsRef<Path>>(posts: &[Post], directory: &I) -> Result<()> {
    let directory = directory.as_ref();

    if !directory.exists() {
        create_dir_all(directory)?;
    }

    for post in posts {
        post.save(directory)?;
    }

    Ok(())
}
