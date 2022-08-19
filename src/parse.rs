use crate::Args;
use crate::{errors::Error, file::File, repo::Repo};
use regex::Regex;
use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;
use std::str;

fn read_file(path: PathBuf) -> Result<String, Error> {
    let bytes = fs::read(path)?;
    let buffer = str::from_utf8(&bytes)?;
    Ok(buffer.to_string())
}

fn parse_file_with_metadata(entry: Result<DirEntry, std::io::Error>) -> Result<File, Error> {
    let entry = entry?;
    File::from(entry)
}

pub fn parse_description(mut file: File, args: &Args) -> File {
    if !args.list {
        return file;
    }
    let description_path = file.path.join(".git").join("description");
    let description = read_file(description_path);
    file.repo = match description {
        Ok(desc) if desc.contains("Unnamed repository") => Some(Repo::local()),
        Ok(desc) => Some(Repo::with_descripion(desc)),
        _ => None,
    };
    file
}

fn parse_config(mut file: File) -> File {
    let config_path = file.path.join(".git").join("config");
    let config = read_file(config_path);
    file.repo = match config {
        Ok(conf) => Regex::new(r#".*(github|gitlab).com[:|/](.*)/(.*)"#)
            .ok()
            .and_then(|regexp| regexp.captures(&conf))
            .map(|data| {
                Repo::from(
                    &data[1],
                    &data[2],
                    &data[3].strip_suffix(".git").unwrap_or(&data[3]),
                )
            })
            .or(Some(Repo::local())),
        _ => None,
    };
    file
}

pub fn parse_files(paths: ReadDir, args: &Args) -> Vec<File> {
    paths
        .map(|path| parse_file_with_metadata(path).unwrap_or(File::new_error_file()))
        .filter(|f| args.all || !f.is_hidden)
        .map(|file| parse_config(file))
        .collect()
}
