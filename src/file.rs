use std::{path::PathBuf, fs::{DirEntry, self}};
use reqwest::{Url, Method};
use serde_derive::Deserialize;
use colored::Colorize;
use crate::{errors::Error, repo::Repo};

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub repo: Option<Repo>,
    pub is_hidden: bool,
    pub name: String,
    pub is_dir: bool,
    pub len: u64
}

#[derive(Deserialize)]
struct GitHubData {
    description: Option<String>,
}


impl File {
    pub fn from(entry: DirEntry) -> Result<Self, Error> {
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
            len: metadata.len()
        })
    }

    pub fn new_error_file() -> Self {
        Self {
            path: PathBuf::new(),
            name: format!("{}", "Unknown path".red()),
            is_hidden: false,
            is_dir: false,
            repo: None,
            len: 0
        }
    }

    pub async fn get_description(&mut self) -> Result<(), Error> {
        if let Some(repo) = self.repo.as_ref() {
            let url = match repo.provider.as_str() {
                "github" => format!("https://api.github.com/repos/{}/{}", repo.user,repo.repo),
                "gitlab" => format!(""),
                _ => panic!("Provider not supported"),
            };
            let url = Url::parse(&*url).map_err(|_| Error::ParseError)?;
            let client = reqwest::Client::new();
            let res = client.request(Method::GET, url)
                .header("User-Agent", "request")
                .send()
                .await?;
            let res = res.json::<GitHubData>().await?;
            let description_path = self.path.join(".git").join("description");
            if let Some(desc) = res.description {
                fs::write(&description_path, desc)?;
            }
        }
        Ok(())
    }
}

