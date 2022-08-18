use crate::{file::File, Args};
use colored::Colorize;

fn format_file(mut file: File, args: &Args) -> String {
    if file.repo.is_some() {
        file.name = format!("{}", file.name.bold().green());
    } else if file.is_dir {
        file.name = format!("{}", file.name.bold().cyan());
    }


    if args.list && file.repo.is_some() {
        let d = file.repo.unwrap().description.unwrap_or("No description".to_string());
        format!("{}, {}\n", file.name, d)
    } else if args.list {
        format!("{}\n", file.name)
    } else {
        format!("{} ", file.name)
    }
}

pub fn format_files(files: Vec<File>, args: &Args) -> Vec<String> {
    files
        .into_iter()
        .map(|file| format_file(file, &args))
        .collect()
}
