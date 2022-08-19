use crate::{
    errors::{Error, FileError},
    repo::Repo,
};
use colored::Colorize;
use reqwest::{Method, Url};
use serde_derive::Deserialize;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub repo: Option<Repo>,
    pub is_hidden: bool,
    pub name: String,
    pub is_dir: bool,
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
        })
    }

    pub fn new_error_file() -> Self {
        Self {
            path: PathBuf::new(),
            name: format!("{}", "Unknown path".red()),
            is_hidden: false,
            is_dir: false,
            repo: None,
        }
    }

    fn write_description(&self, path: PathBuf, desc: String) -> Result<(), Error> {
        fs::write(&path, desc)?;
        Ok(())
    }

    pub async fn get_description(&mut self) -> Result<&File, FileError> {
        if let Some(repo) = self.repo.as_mut() {
            let description_path = self.path.join(".git").join("description");
            if !repo.remote {
                self.write_description(
                    description_path,
                    format!("{}", "Repository not linked with origin".red()),
                )
                .map_err(|e| FileError::IOError(self, e))?;
                return Ok(self);
            }
            let url = match repo.provider.as_str() {
                "github" => format!("https://api.github.com/repos/{}/{}", repo.user, repo.repo),
                "gitlab" => format!(
                    "https://gitlab.com/api/v4/projects/{}%2F{}",
                    repo.user, repo.repo
                ),
                _ => panic!("Provider not supported"),
            };
            let url =
                Url::parse(&*url).map_err(|_| FileError::ParseError(self, Error::ParseError))?;
            let client = reqwest::Client::new();
            let res = client
                .request(Method::GET, url)
                .header("User-Agent", "request")
                .send()
                .await
                .map_err(|e| FileError::ReqwestError(self, Error::ReqwestError(e)))?;
            let res = res
                .json::<GitHubData>()
                .await
                .map_err(|e| FileError::DeserializeError(self, Error::ReqwestError(e)))?;
            if let Some(desc) = res.description {
                self.write_description(description_path, desc)
                    .map_err(|e| FileError::IOError(self, e))?;
            }
        }
        Ok(self)
    }
}
