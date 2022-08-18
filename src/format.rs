use crate::{file::File, Args};
use colored::Colorize;

fn format_file(file: File, args: &Args) -> String {
    let mut display = file.name;
    if file.repo.is_some() {
        display = format!("{}", display.bold().green());
    } else if file.is_dir {
        display = format!("{}", display.bold().cyan());
    }

    if args.list {
        display = format!("{:>5}  {}", file.len, display);

        if file.repo.is_some() {
            let description = file.repo.unwrap().description.unwrap_or("No description".to_string());
            display = format!("{}, {}", display, description);
        }
        format!("{}\n", display)
    } else {
        format!("{} ", display)
    }
}

pub fn format_files(files: Vec<File>, args: &Args) -> Vec<String> {
    files
        .into_iter()
        .map(|file| format_file(file, &args))
        .collect()
}
