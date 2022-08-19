mod errors;
mod file;
mod format;
mod parse;
mod repo;
mod sync;

use clap::Parser;
use format::format_files;
use parse::{parse_description, parse_files};
use std::fs;
use sync::sync_files;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_parser)]
    path: Option<String>,

    /// Do not ignore entries starting with .
    #[clap(short, long, action)]
    all: bool,

    /// Display files
    #[clap(short, long, action)]
    list: bool,

    /// Sync GitHub and GitLab repository description with git description
    #[clap(short, long, action)]
    sync: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let pwd = args.path.clone().unwrap_or(".".to_string());
    let paths = fs::read_dir(&pwd).unwrap();

    let mut files = parse_files(paths, &args);
    files.sort_by_key(|f| f.path.clone());

    let files = sync_files(files, &args).await;

    let files = files
        .into_iter()
        .map(|f| parse_description(f, &args))
        .collect();

    let files = format_files(files, &args);

    if args.list {
        println!("{}", files.join("\n"));
    } else {
        println!("{}", files.join(" "));
    }
}
