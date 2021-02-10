use anyhow::Result;
use fs_extra::{copy_items, dir};

pub fn copy_assets(source: &str, target: &str) -> Result<()> {
    let options = dir::CopyOptions::new();
    let from = vec![source];

    copy_items(&from, target, &options)?;

    Ok(())
}
