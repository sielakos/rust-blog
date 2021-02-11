use crate::posts::Post;
use anyhow::Result;
use std::{fs::File, io::Write, path::Path};
use tera::{Context as TeraContext, Tera};

pub fn build_index<I: AsRef<Path>>(posts: &[Post], tera: &Tera, target: &I) -> Result<()> {
    let target = target.as_ref();
    let mut context = TeraContext::new();

    context.insert("posts", &posts);
    let content = tera.render("index.html", &context)?;

    let mut file = File::create(target.join("index.html"))?;

    file.write_all(content.as_bytes())?;
    file.flush()?;

    Ok(())
}
