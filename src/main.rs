mod cli;
use colored::*;
use directories::ProjectDirs;
use open;
use prettyprint::*;
use std::io::Write;
use std::{env, fs, io, path, process};

fn main() {
    let cli_args = cli::build_cli().get_matches();
    let data_dir = ProjectDirs::from("", "", "mn").unwrap();
    let data_dir = data_dir.data_local_dir().to_str().unwrap();
    if let Some(in_file) = cli_args.value_of("MNEMONIC") {
        let file = format!("{}/{}.md", data_dir, in_file);
        if cli_args.is_present("edit") {
            if path::Path::new(&file).exists() {
                if let Some(editor) = env::var_os("VISUAL") {
                    process::Command::new(editor).arg(&file).status().unwrap();
                } else if let Some(editor) = env::var_os("EDITOR") {
                    process::Command::new(editor).arg(&file).status().unwrap();
                } else {
                    open::that(&file).is_ok();
                }
            } else {
                eprintln!(
                    "{} not found.  Would you like to add it to Mnemonic?",
                    in_file.yellow().bold()
                );
                process::exit(1);
            }
        } else if cli_args.is_present("new") {
            if !path::Path::new(&file).exists() {
                if let Some(editor) = env::var_os("VISUAL") {
                    process::Command::new(editor).arg(file).status().unwrap();
                } else if let Some(editor) = env::var_os("EDITOR") {
                    process::Command::new(editor).arg(file).status().unwrap();
                } else {
                    open::that(file).is_ok();
                }
            } else {
                eprintln!(
                    "{} already exists.  Did you mean to edit it instead?",
                    in_file.yellow().bold()
                );
                process::exit(1);
            }
        } else if cli_args.is_present("add") {
            if !path::Path::new(&file).exists() {
                fs::File::create(&file).unwrap();
                eprintln!("{} created.", in_file.blue());
            } else {
                eprintln!(
                    "{} already exists.  Did you mean to edit it instead?",
                    in_file.yellow().bold()
                );
                process::exit(1);
            }
        } else if cli_args.is_present("rm") {
            println!(
                "Are you sure you want to delete {}? [y/n]",
                in_file.yellow().bold()
            );
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            loop {
                match &answer[..] {
                    "y\n" | "yes\n" => {
                        fs::remove_file(file).unwrap_or_else(|e| {
                            eprintln!(
                                "There was an error deleting {}:\n{}",
                                in_file.yellow().bold(),
                                e
                            );
                            process::exit(2);
                        });
                        println!("{} has been deleted.", in_file.blue());
                        break;
                    }
                    "n\n" | "no\n" => {
                        break;
                    }
                    _ => {
                        println!("Please type 'yes' ('y') or 'no' ('n')");

                        io::stdin().read_line(&mut answer).unwrap();
                    }
                }
            }
        } else if cli_args.is_present("push") {
            let mut mnemonic_file = fs::OpenOptions::new()
                .append(true)
                .open(file)
                .unwrap_or_else(|_| {
                    eprintln!(
                        "{} not found.  Would you like to add it to Mnemonic?",
                        in_file.yellow().bold()
                    );
                    process::exit(1);
                });

            let new_line = cli_args.value_of("push").unwrap();
            mnemonic_file
                .write(format!("\n{}", new_line).as_bytes())
                .unwrap();
        } else {
            let theme = cli_args.value_of("theme").unwrap_or("TwoDark");
            let printer = PrettyPrinter::default()
                .header(false)
                .grid(false)
                .language("md")
                .theme(theme)
                .line_numbers(false)
                .build()
                .unwrap();

            printer.file(file).unwrap_or_else(|_| {
                eprintln!(
                    "{} not found.  Would you like to add it to Mnemonic?",
                    in_file.yellow().bold()
                );
                process::exit(1);
            });
        }
    } else if cli_args.is_present("list") {
        let mut file_list = vec![];
        for file in fs::read_dir(data_dir).unwrap() {
            file_list.push(format!(
                "  - {}",
                file.unwrap()
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
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
}
