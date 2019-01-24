mod cli;
use clap::{ArgMatches, Error, ErrorKind};
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
        .expect("Should be able to find local data directory inside project directory")
        .to_string();

    fs::create_dir_all(&data_dir)
        .expect("should be able to create the data directory if it does not already exist");

    match cli_args.subcommand() {
        // TODO: let user rm multiple files in one command
        ("rm", Some(rm_args)) => rm(rm_args, &data_dir),
        // TODO: let user specify syntax for mnemonic
        ("add", Some(add_args)) => add(add_args, &data_dir),
        ("list", Some(_list_args)) => list(&data_dir),
        // TODO: let user edit syntax for mnemonic
        ("edit", Some(edit_args)) => edit(edit_args, &data_dir),
        // TODO let user specify plain text or other syntax
        ("show", Some(show_args)) => show(&show_args, &data_dir, &cli_args),
        _ => show(&cli_args, &data_dir, &cli_args),
    }
}

fn edit(edit_args: &ArgMatches, data_dir: &String) {
    let file_name = edit_args.value_of("MNEMONIC").unwrap_or_else(|| {
        err_no_mnemonic();
        unreachable!();
    });
    let full_path = format!("{}/{}.md", data_dir, file_name);
    if path::Path::new(&full_path).exists() {
        if let Some(text_to_append) = edit_args.value_of("push") {
            append_to_mnemonic(&full_path, &file_name, text_to_append);
        } else if let Some(editor) = env::var_os("VISUAL") {
            process::Command::new(editor)
                .arg(&full_path)
                .status()
                .expect("should be able to open file with $VISUAL");
        } else if let Some(editor) = env::var_os("EDITOR") {
            process::Command::new(editor)
                .arg(&full_path)
                .status()
                .expect("should be able to open file with $EDITOR");
        } else {
            open::that(&full_path).is_ok();
        }
    } else {
        eprintln!(
            "{} not found.  Would you like to add it to Mnemonic?",
            file_name.yellow().bold()
        );
        process::exit(1);
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
        println!("'{}' added to {}", text_to_append, file_name.blue());
    }
}

fn add(add_args: &ArgMatches, data_dir: &String) {
    let file_name = add_args.value_of("MNEMONIC").unwrap_or_else(|| {
        err_no_mnemonic();
        unreachable!();
    });
    let full_path = format!("{}/{}.md", data_dir, file_name);
    if !path::Path::new(&full_path).exists() {
        fs::File::create(&full_path).unwrap();
        if add_args.is_present("blank") {
            eprintln!("{} created.", file_name.blue());
        } else {
            edit(add_args, &data_dir);
        }
    } else {
        eprintln!(
            "{} already exists.  Did you mean to edit it instead?",
            file_name.yellow().bold()
        );
        process::exit(1);
    }
    process::exit(0);
}

fn rm(rm_args: &ArgMatches, data_dir: &String) {
    let file_name = rm_args.value_of("MNEMONIC").unwrap_or_else(|| {
        err_no_mnemonic();
        unreachable!();
    });
    let full_path = format!("{}/{}.md", data_dir, file_name);
    if !path::Path::new(&full_path).exists() {
        eprintln!("No mnemonic named {} exists", file_name.yellow().bold());
        process::exit(1);
    }
    if rm_args.is_present("force") {
        delete_file(full_path, file_name);
    } else {
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
                    delete_file(full_path, file_name);
                    break;
                }
                "n\n" | "no\n" => {
                    break;
                }
                _ => {
                    println!("Please type 'yes' ('y') or 'no' ('n')");
                    answer = String::new();
                    io::stdin()
                        .read_line(&mut answer)
                        .expect("Should be able to read input from stdin");
                }
            }
        }
    }
    fn delete_file(full_path: String, file_name: &str) {
        fs::remove_file(full_path).unwrap_or_else(|e| {
            eprintln!(
                "There was an error deleting {}:\n{}",
                file_name.yellow().bold(),
                e
            );
            process::exit(2);
        });
        println!("{} has been deleted.", file_name.blue());
    }
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

fn list(data_dir: &str) {
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

fn show(show_args: &ArgMatches, data_dir: &str, cli_args: &ArgMatches) {
    cli_args.value_of("MNEMONIC").unwrap_or_else(|| {
        err_no_mnemonic();
        unreachable!();
    });
    if let Some(usr_supplied_file_name) = cli_args.value_of("MNEMONIC") {
        let full_path = format!("{}/{}.md", &data_dir, usr_supplied_file_name);
        let theme = show_args.value_of("theme").unwrap_or("TwoDark");
        print_mnemonic(full_path, &usr_supplied_file_name, theme);
    }
}

fn err_no_mnemonic() {
    let err = Error::with_description(
        "MNEMONIC is a required argument\nFor more information, use the help command",
        ErrorKind::MissingRequiredArgument,
    );
    Error::exit(&err)
}
