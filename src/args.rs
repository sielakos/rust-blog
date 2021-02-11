use crate::{build, server, watch};
use actix;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sielakos-blog cli")]
pub struct Args {
    #[structopt(short, long)]
    pub watch: bool,

    #[structopt(short, long)]
    pub serve: bool,

    #[structopt(short = "T", long)]
    pub templates: Option<String>,

    #[structopt(short, long, parse(from_os_str))]
    pub posts: Option<PathBuf>,

    #[structopt(short, long, parse(from_os_str))]
    pub assets: Option<PathBuf>,

    #[structopt(short, long, parse(from_os_str))]
    pub target: Option<PathBuf>,
}

impl Args {
    pub async fn run() -> Result<()> {
        let args: Args = Args::from_args();
        let templates = args.templates.unwrap_or("templates".to_owned());
        let posts = args.posts.unwrap_or(PathBuf::from("./posts"));
        let assets = args.assets.unwrap_or(PathBuf::from("./assets"));
        let target = args.target.unwrap_or(PathBuf::from("./build"));
        let templates_path = PathBuf::from(format!("./{}", templates));
        let templates = format!("{}/**/*", templates);

        build::build(&posts, &target, &assets, &templates)?;

        if args.watch {
            let posts = posts.clone();
            let target = target.clone();
            let assets = assets.clone();

            actix::spawn(async move {
                watch::watch(
                    {
                        let posts = posts.clone();
                        let target = target.clone();
                        let assets = assets.clone();

                        move || -> Result<()> { build::build(&posts, &target, &assets, &templates) }
                    },
                    &posts,
                    &assets,
                    &templates_path,
                )
                .expect("Failed to watch");
            });
        }

        if args.serve {
            server::run(target).await?;
        }

        Ok(())
    }
}
