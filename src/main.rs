mod cli;
use colored::*;
use directories::ProjectDirs;
use open;
use prettyprint::*;
use std::io::Write;
use std::{env, fs, io, path, process};

fn main() {
    let cli_args = cli::build_cli().get_matches();
    let data_dir =
        ProjectDirs::from("", "", "mn").expect("Should be able to determine project directory");
    let data_dir = data_dir
        .data_local_dir()
        .to_str()
        .expect("Should be able to find local data directory inside project directory");
    fs::create_dir_all(data_dir)
        .expect("should be able to create the data directory if it does not already exist");
    if let Some(usr_supplied_file_name) = cli_args.value_of("MNEMONIC") {
        let full_path = format!("{}/{}.md", data_dir, usr_supplied_file_name);
        if cli_args.is_present("edit") {
            edit_mnemonic(&full_path, &usr_supplied_file_name);
        } else if cli_args.is_present("new") {
            create_new_mnemonic(&full_path, &usr_supplied_file_name);
        } else if cli_args.is_present("add") {
            add_new_mnemonic(&full_path, &usr_supplied_file_name);
        } else if cli_args.is_present("rm") {
            delete_mnemonic(&full_path, &usr_supplied_file_name);
        } else if cli_args.is_present("push") {
            let text_to_append = cli_args
                .value_of("push")
                .expect("Guaranteed to exist based on `is_present`");
            append_to_mnemonic(&full_path, &usr_supplied_file_name, text_to_append);
        } else {
            let theme = cli_args.value_of("theme").unwrap_or("TwoDark");
            print_mnemonic(full_path, &usr_supplied_file_name, theme);
        }
    } else if cli_args.is_present("list") {
        list_mnemonics(&data_dir)
    }
}

fn edit_mnemonic(file_path: &String, file_name: &str) {
    if path::Path::new(&file_path).exists() {
        if let Some(editor) = env::var_os("VISUAL") {
            process::Command::new(editor)
                .arg(&file_path)
                .status()
                .expect("should be able to open file with $VISUAL");
        } else if let Some(editor) = env::var_os("EDITOR") {
            process::Command::new(editor)
                .arg(&file_path)
                .status()
                .expect("should be able to open file with $EDITOR");
        } else {
            open::that(&file_path).is_ok();
        }
    } else {
        eprintln!(
            "{} not found.  Would you like to add it to Mnemonic?",
            file_name.yellow().bold()
        );
        process::exit(1);
    }
}

fn create_new_mnemonic(file_path: &String, file_name: &str) {
    if !path::Path::new(&file_path).exists() {
        if let Some(editor) = env::var_os("VISUAL") {
            fs::File::create(&file_path)
                .expect("should be able to create a new file in the local data directory");
            process::Command::new(editor)
                .arg(file_path)
                .status()
                .expect("should be able to open file with $VISUAL");
        } else if let Some(editor) = env::var_os("EDITOR") {
            fs::File::create(&file_path)
                .expect("should be able to create a new file in the local data directory");
            process::Command::new(editor)
                .arg(file_path)
                .status()
                .expect("should be able to open file with $EDITOR");
        } else {
            fs::File::create(&file_path)
                .expect("should be able to create a new file in the local data directory");
            if open::that(file_path).is_err() {
                eprintln!(
                    "Could not open {}.  Do you have read and write access to {}?",
                    file_name.yellow().bold(),
                    file_path.yellow().bold(),
                );
            }
        }
    } else {
        eprintln!(
            "{} already exists.  Did you mean to edit it instead?",
            file_name.yellow().bold()
        );
        process::exit(1);
    }
}

fn add_new_mnemonic(file_path: &String, file_name: &str) {
    if !path::Path::new(&file_path).exists() {
        fs::File::create(&file_path).unwrap();
        eprintln!("{} created.", file_name.blue());
    } else {
        eprintln!(
            "{} already exists.  Did you mean to edit it instead?",
            file_name.yellow().bold()
        );
        process::exit(1);
    }
}

fn delete_mnemonic(file_path: &String, file_name: &str) {
    println!(
        "Are you sure you want to delete {}? [y/n]",
        file_name.yellow().bold()
    );
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Should be able to read input from stdin");
    loop {
        match &answer[..] {
            "y\n" | "yes\n" => {
                fs::remove_file(file_path).unwrap_or_else(|e| {
                    eprintln!(
                        "There was an error deleting {}:\n{}",
                        file_name.yellow().bold(),
                        e
                    );
                    process::exit(2);
                });
                println!("{} has been deleted.", file_name.blue());
                break;
            }
            "n\n" | "no\n" => {
                break;
            }
            _ => {
                println!("Please type 'yes' ('y') or 'no' ('n')");

                io::stdin()
                    .read_line(&mut answer)
                    .expect("Should be able to read input from stdin");
            }
        }
    }
}

fn append_to_mnemonic(file_path: &String, file_name: &str, text_to_append: &str) {
    let mut mnemonic_file = fs::OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap_or_else(|_| {
            eprintln!(
                "{} not found.  Would you like to add it to Mnemonic?",
                file_name.yellow().bold()
            );
            process::exit(1);
        });

    mnemonic_file
        .write(format!("\n{}", text_to_append).as_bytes())
        .expect("Should be able to write to mnemonic file");
}

fn print_mnemonic(file_path: String, file_name: &str, theme: &str) {
    let printer = PrettyPrinter::default()
        .header(false)
        .grid(false)
        .language("md")
        .theme(theme)
        .line_numbers(false)
        .build()
        .expect("should be able to build a formater");

    printer.file(file_path).unwrap_or_else(|_| {
        eprintln!(
            "{} not found.  Would you like to add it to Mnemonic?",
            file_name.yellow().bold()
        );
        process::exit(1);
    });
}

fn list_mnemonics(data_dir: &str) {
    let mut file_list = vec![];
    for file in fs::read_dir(data_dir).expect("Should be able to read the local data directory") {
        file_list.push(format!(
            "  - {}",
            file.expect("file should exist")
                .path()
                .file_stem()
                .expect("file should have valid stem")
                .to_str()
                .expect("file should be able to be converted to a string")
                .blue()
                .bold()
        ));
    }

    println!("Your {} available mnemonics are:", file_list.len());
    file_list.sort();
    for line in file_list {
        println!("{}", line);
    }
}
