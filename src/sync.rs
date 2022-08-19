use crate::{errors::FileError, file::File, Args};
use colored::Colorize;
use futures::future;

fn print_error(message: &str, filename: &str) {
    println!("{} {} for {}", "Error!".bold().red(), message, filename);
}

pub async fn sync_files(mut files: Vec<File>, args: &Args) -> Vec<File> {
    if args.sync {
        println!("{}", "Syncing, please wait...".cyan());
        let mut repositories: Vec<_> = files
            .iter_mut()
            .map(|f| f.get_description())
            .map(Box::pin)
            .collect();

        while !repositories.is_empty() {
            let (result, _, remaining) = future::select_all(repositories).await;
            match result {
                Ok(file) if file.repo.is_some() => {
                    println!("{} {}", "Done!".bold().cyan(), file.name)
                }
                Ok(_) => {}
                Err(FileError::ReqwestError(file, _)) => {
                    print_error("Error syncing repo", &file.name)
                }
                Err(FileError::IOError(file, _)) => {
                    print_error("Error saving description", &file.name)
                }
                Err(FileError::ParseError(file, _)) => print_error("Error parsing url", &file.name),
                Err(FileError::DeserializeError(file, _)) => {
                    print_error("Error deserializing response", &file.name)
                }
            }
            repositories = remaining;
        }
        println!("\n");
    }
    files
}
