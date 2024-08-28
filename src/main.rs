use std::path::Path;

use clap::{Arg, Command};
use rusty_roots::directory::Directory;

fn main() {
    let matches = Command::new("rusty-roots")
        .version("1.0")
        .about("Rust-powered CLI to display directory tree")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Path to get tree from"),
        )
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .action(clap::ArgAction::SetTrue)
                .help("Ignore files and directories as specified in {$path}/.rrignore"),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .action(clap::ArgAction::SetTrue)
                .help("Do not stylize tree text output"),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("path");
    let ignore = matches.get_flag("ignore");
    let color = !matches.get_flag("no-color");
    let target_path = match input_path {
        Some(s) => Path::new(s),
        None => Path::new("."),
    };

    if color {
        let dir = Directory::new(target_path, ignore).unwrap();
        dir.print_body().unwrap();
    } else {
        let dir = Directory::new_with_empty_body(target_path, ignore).unwrap();
        dir.fast_print_body().unwrap();
    }
}
