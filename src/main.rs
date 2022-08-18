mod errors;
mod file;
mod format;
mod parse;
mod repo;
mod sync;

use format::format_files;
use parse::parse_files;
use clap::Parser;
use sync::sync_files;
use std::fs;

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
    let files = format_files(files, &args);
    files.into_iter().for_each(|f| print!("{}", f))
}
