use std::fs;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn oneline(path: &str, show_hidden: bool) {
    let mut file_entries = fs::read_dir(path)
        .expect("Couldn't read directory.")
        .collect::<Vec<_>>();
    file_entries.sort_by(|a, b| {
        a.as_ref()
            .expect("Couldn't read to a.")
            .file_name()
            .cmp(&b.as_ref().expect("Couldn't read to b.").file_name())
    });

    for entries in file_entries {
        let entry = entries.expect("Couldn't read entry.");
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !show_hidden && name.starts_with(".") {
            continue;
        }

        if entry.path().is_dir() {
            print!("{}{}{}{} ", BOLD, BLUE, name, RESET);
        } else {
            print!("{} ", name);
        }
    }
    println!("");
}

pub fn tree(path: &str, show_hidden: bool, prefix: String, depth: u32, level: u32) {
    if (level != 0) & (depth == level) {
        return;
    }

    let mut file_entries = fs::read_dir(path)
        .expect("Couldn't read directory.")
        .collect::<Vec<_>>();
    file_entries.sort_by(|a, b| {
        a.as_ref()
            .expect("Couldn't read to a.")
            .file_name()
            .cmp(&b.as_ref().expect("Couldn't read to b.").file_name())
    });

    for (index, entries) in file_entries.iter().enumerate() {
        let entry = entries.as_ref().expect("Couldn't read entry.");
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !show_hidden && name.starts_with(".") {
            continue;
        }

        if index == file_entries.len() - 1 {
            println!("{}└── {}", prefix, name);
            if entry.path().is_dir() {
                let prefix_new = prefix.clone() + "     ";
                tree(
                    entry.path().to_str().unwrap(),
                    show_hidden,
                    prefix_new,
                    depth + 1,
                    level,
                );
            }
        } else {
            println!("{}├── {}", prefix, name);
            if entry.path().is_dir() {
                let prefix_new = prefix.clone() + "|    ";
                tree(
                    entry.path().to_str().unwrap(),
                    show_hidden,
                    prefix_new,
                    depth + 1,
                    level,
                )
            }
        }
    }
}
