use crate::posts;
use anyhow::Result;

pub fn build() -> Result<()> {
    let iter = posts::read_posts("./inputs")?;

    posts::save_posts(iter, "./outputs")?;

    Ok(())
}
