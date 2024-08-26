use std::path::Path;

use colored::{ColoredString, Colorize};
use clap::builder::OsStr;
use mime_guess::MimeGuess;


pub fn color_path(p: &Path) -> ColoredString {
    let cur_dir = OsStr::from(".");
    let name = p.file_name().unwrap_or(&cur_dir);
    let name = name.to_str().unwrap();
    if p.is_dir() {
        return name.blue().bold();
    } 
    let filetype = MimeGuess::from_path(p).first();

    match filetype {
        Some(mime) if mime.type_() == "image" => name.bright_magenta(),
        Some(mime) if mime.type_() == "application" => name.bright_green(),
        _ => name.normal()
    }
}