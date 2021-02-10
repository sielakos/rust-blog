#[macro_use]
extern crate lazy_static;

mod format;
mod highlight;
mod posts;
mod templates;

fn main() {
    let iter = posts::read_posts("./inputs").expect("Failed to read posts");

    posts::save_posts(iter, "./outputs").expect("Failed to save posts");
}
