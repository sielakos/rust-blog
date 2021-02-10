use crate::format::format_markdown;
use crate::templates::TEMPLATES;
use anyhow::{Context, Result};
use pulldown_cmark::{Event, Parser};
use std::{
    fs::{create_dir_all, read_dir, File},
    io::{Read, Write},
    path::Path,
};
use tera::Context as TeraContext;

pub struct Post {
    content: String,
    title: String,
    filename: String,
}

impl Post {
    pub fn save(&self, directory: &Path) -> Result<()> {
        let path = directory.join(format!("{}.{}", &self.filename, "html"));
        let mut file = File::create(path)?;

        file.write_all(self.content.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn read(path: &Path) -> Result<Post> {
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

        let content = TEMPLATES.render("document.html", &context)?;

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

pub fn read_posts(directory: &str) -> Result<impl Iterator<Item = Result<Post>>> {
    let res = read_dir(directory)?.map(|file| {
        let file = file?;

        Post::read(&file.path())
    });

    Ok(res)
}

pub fn save_posts<I>(posts: I, directory: &str) -> Result<()>
where
    I: Iterator<Item = Result<Post>>,
{
    let directory = Path::new(directory);

    if !directory.exists() {
        create_dir_all(directory)?;
    }

    for post in posts {
        let post = post?;

        post.save(directory)?;
    }

    Ok(())
}
