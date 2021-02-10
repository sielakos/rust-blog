use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch<I: Fn() -> Result<()>>(callback: I) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch("./inputs", RecursiveMode::Recursive)?;
    watcher.watch("./assets", RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        rx.recv()?;
        println!("rebuilding :)");
        callback()?;
    }
}
