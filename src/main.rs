extern crate clap;

mod lib;
use clap::{Arg, ArgAction, Command};
use dls::{list_directory, list_directory_tree};

fn main() {
    let matches = Command::new("dls")
        .about("dumb listing directory program")
        .version("0.1.0")
        .arg(
            Arg::new("path")
                .help("Path to the directory list.")
                .required(false)
                .default_value("."),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Show all files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("level")
                .short('L')
                .long("level")
                .value_name("DEPTH")
                .help("Limit recursion depth in some display option")
                .default_value("0")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("tree")
                .short('T')
                .long("tree")
                .help("List in tree format")
                .conflicts_with("oneline")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("oneline")
                .short('1')
                .long("oneline")
                .help("List in oneline")
                .conflicts_with("tree")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let path: String = matches.get_one::<String>("path").unwrap().to_string();

    let show_hidden = matches.get_flag("all");
    let level: String = matches.get_one::<String>("level").unwrap().to_string();
    let level: u32 = level.parse().unwrap();

    if matches.get_flag("tree") {
        list_directory_tree(&path, show_hidden, String::from(""), 0, level);
    } else if matches.get_flag("oneline") || !matches.get_flag("oneline") {
        list_directory(&path, show_hidden);
    }
}
