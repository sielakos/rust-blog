use anyhow::Result;
use fs_extra::{copy_items, dir};
use std::path::Path;

pub fn copy_assets<I: AsRef<Path>>(source: &I, target: &I) -> Result<()> {
    let mut options = dir::CopyOptions::new();
    let from = vec![source];

    options.overwrite = true;

    copy_items(&from, target, &options)?;

    Ok(())
}
