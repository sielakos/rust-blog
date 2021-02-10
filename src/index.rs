use crate::posts::Post;
use crate::templates::TEMPLATES;
use anyhow::Result;
use std::{fs::File, io::Write};
use tera::Context as TeraContext;

pub fn build_index(posts: &[Post]) -> Result<()> {
    let mut context = TeraContext::new();

    context.insert("posts", &posts);
    let content = TEMPLATES.render("index.html", &context)?;

    let mut file = File::create("./outputs/index.html")?;

    file.write_all(content.as_bytes())?;
    file.flush()?;

    Ok(())
}
