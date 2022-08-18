use std::fs::{DirEntry, self, ReadDir};
use std::path::PathBuf;
use std::str;
use regex::Regex;
use crate::Args;
use crate::{repo::Repo,file::File, errors::Error};

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
    file.repo = match description {
        Ok(desc) if desc.contains("Unnamed repository") => Some(Repo::local()),
        Ok(desc) => Some(Repo::with_descripion(desc)),
        _ => None,
    };
    file
}

fn parse_config(mut file: File, args: &Args) -> File {
    if !args.sync {
        return file;
    }
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


pub fn parse_files(paths: ReadDir, args: &Args) -> Vec<File> {
    paths
        .map(|path| parse_file_with_metadata(path).unwrap_or(File::new_error_file()))
        .filter(|f| args.all || !f.is_hidden)
        .map(|file| parse_description(file, &args))
        .map(|file| parse_config(file, &args))
        .collect()
}
