use crate::{assets, index, posts};
use anyhow::Result;
use fs_extra::dir::remove;

pub fn build() -> Result<()> {
    let posts = posts::read_posts("./inputs")?.collect::<Result<Vec<_>>>()?;

    remove("./outputs")?;
    posts::save_posts(&posts, "./outputs")?;
    assets::copy_assets("./assets", "./outputs")?;
    index::build_index(&posts)?;

    Ok(())
}
