use clap::Parser;
use colored::Colorize;
use futures::future;
use regex::Regex;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::fs::DirEntry;
use std::str::{self, Utf8Error};
use std::{ffi::OsString, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
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

enum Error {
    IOError(std::io::Error),
    OsStringError(OsString),
    Utf8Error(Utf8Error),
    ParseError,
    ReqwestError(reqwest::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOError(e)
    }
}

impl From<OsString> for Error {
    fn from(e: OsString) -> Error {
        Error::OsStringError(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::Utf8Error(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::ReqwestError(e)
    }
}

struct Repo {
    remote: bool,
    provider: String,
    user: String,
    repo: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubData {
    description: String,
}

impl Repo {
    fn from(provider: &str, user: &str, repo: &str) -> Self {
        Self {
            remote: true,
            provider: String::from(provider),
            user: String::from(user),
            repo: String::from(repo),
        }
    }

    fn local() -> Self {
        Self {
            remote: false,
            provider: "".to_string(),
            user: "".to_string(),
            repo: "".to_string(),
        }
    }

    async fn get(&self) -> future::Result<GitHubData, Error> {
        let url = match self.provider.as_str() {
            "github" => format!("https://api.github.com/repos/{}/{}", self.user, self.repo),
            "gitlab" => format!(""),
            _ => panic!("Provider not supported"),
        };

        let url = Url::parse(&*url).map_err(|_| Error::ParseError)?;
        let res = reqwest::get(url).await?.json::<GitHubData>().await?;
        dbg!(&res);
        Ok(res)
    }
}

struct File {
    path: PathBuf,
    name: String,
    is_hidden: bool,
    is_dir: bool,
    repo: Option<Repo>,
    description: Option<String>,
}

impl File {
    fn from(entry: DirEntry) -> Result<Self, Error> {
        let metadata = entry.metadata()?;
        let filename = entry.file_name().into_string()?;
        let is_dir = metadata.is_dir();
        let is_hidden = filename.starts_with(".");
        Ok(Self {
            path: entry.path(),
            name: filename,
            is_hidden,
            is_dir,
            repo: None,
            description: None,
        })
    }

    fn new_error_file() -> Self {
        Self {
            path: PathBuf::new(),
            name: format!("{}", "Unknown path".red()),
            is_hidden: false,
            is_dir: false,
            repo: None,
            description: None,
        }
    }
}

fn read_file(path: PathBuf) -> Result<String, Error> {
    let bytes = fs::read(path)?;
    let buffer = str::from_utf8(&bytes)?;
    Ok(buffer.to_string())
}

fn parse_file_with_metadata(entry: Result<DirEntry, std::io::Error>) -> Result<File, Error> {
    let entry = entry?;
    File::from(entry)
}

fn parse_description(mut file: File, args: &Args) -> File {
    if !args.list {
        return file;
    }
    let description_path = file.path.join(".git").join("description");
    let description = read_file(description_path);
    file.description = match description {
        Ok(desc) if desc.contains("Unnamed repository") => None,
        Ok(desc) => Some(desc),
        _ => None,
    };
    file
}

fn parse_config(mut file: File, args: &Args) -> File {
    let config_path = file.path.join(".git").join("config");
    let config = read_file(config_path);
    file.repo = match config {
        Ok(conf) => Regex::new(r#".*(github|gitlab).*:(.*)/(.*).git"#)
            .ok()
            .and_then(|regexp| regexp.captures(&conf))
            .map(|data| Repo::from(&data[1], &data[2], &data[3]))
            .or(Some(Repo::local())),
        _ => None,
    };
    file
}

fn format_file(file: File, args: &Args) -> String {
    if args.list && file.repo.is_some() {
        let d = file.description.unwrap_or("No description".to_string());
        format!("{}, {}\n", file.name.bold().green(), d)
    } else if args.list && file.is_dir && !file.repo.is_some() {
        format!("{}\n", file.name.bold().cyan())
    } else {
        format!("{} ", file.name)
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let pwd = args.path.clone().unwrap_or(".".to_string());

    let paths = fs::read_dir(&pwd).unwrap();

    let files: Vec<File> = paths
        .map(|path| parse_file_with_metadata(path).unwrap_or(File::new_error_file()))
        .filter(|f| args.all || !f.is_hidden)
        .map(|file| parse_description(file, &args))
        .map(|file| parse_config(file, &args))
        .collect();

    let futures = files
        .iter()
        .filter(|f| f.repo.is_some())
        .map(|f| f.repo.and_then(|r| r.get()));

    future::try_join_all(futures).await;

    let mut files: Vec<String> = files
        .into_iter()
        .map(|file| format_file(file, &args))
        .collect();

    files.sort();
    files.into_iter().for_each(|f| print!("{}", f))
}
