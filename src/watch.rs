use anyhow::{Context, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::{path::Path, time::Duration};

pub fn watch<I, P>(callback: I, posts: &P, assets: &P, templates: &P) -> Result<()>
where
    I: Fn() -> Result<()>,
    P: AsRef<Path>,
{
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, Duration::from_secs(2)).context("Could not create watcher")?;

    watcher
        .watch(posts, RecursiveMode::Recursive)
        .context("Could not watch posts")?;
    watcher
        .watch(assets, RecursiveMode::Recursive)
        .context("Could not watch assets")?;
    watcher
        .watch(templates, RecursiveMode::Recursive)
        .context("Could not watch templates")?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        rx.recv()?;
        println!("rebuilding :)");
        callback()?;
    }
}
