use crate::templates::init_tera;
use crate::{assets, index, posts};
use anyhow::Result;
use std::path::PathBuf;

pub fn build(posts: &PathBuf, target: &PathBuf, assets: &PathBuf, templates: &str) -> Result<()> {
    let tera = init_tera(&templates);
    let posts = posts::read_posts(posts, &tera)?;

    posts::save_posts(&posts, target)?;
    assets::copy_assets(assets, target)?;
    index::build_index(&posts, &tera, target)?;

    Ok(())
}
