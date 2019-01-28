mod cli;
use clap::ArgMatches;
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

    let result = match cli_args.subcommand() {
        // TODO: let user rm multiple files in one command
        ("rm", Some(rm_args)) => rm(rm_args, &data_dir),
        // TODO: let user specify syntax for mnemonic
        ("add", Some(add_args)) => add(add_args, &data_dir),
        ("list", Some(_list_args)) => list(&data_dir),
        // TODO: let user edit syntax for mnemonic
        ("edit", Some(edit_args)) => edit(edit_args, &data_dir),
        // TODO let user specify plain text or other syntax
        ("show", Some(show_args)) => show(&show_args, &data_dir),
        _ => show(&cli_args, &data_dir),
    };

    match result {
        Ok(Some(msg)) => println!("{}", msg),
        Ok(None) => (),
        Err(err) => {
            eprintln!("{}", err.1);
            process::exit(err.0)
        }
    }
}

fn edit(edit_args: &ArgMatches, data_dir: &str) -> Result<Option<String>, (i32, String)> {
    let file_name = edit_args.value_of("MNEMONIC").expect("Required by clap");
    let full_path = format!("{}/{}.md", data_dir, file_name);
    let editor = if let Some(editor) = env::var_os("VISUAL") {
        Some(editor)
    } else {
        env::var_os("EDITOR")
    };
    if path::Path::new(&full_path).exists() {
        if let Some(text_to_append) = edit_args.value_of("push") {
            Ok(append_to_mnemonic(&full_path, &file_name, text_to_append))
        } else if let Some(editor) = editor {
            process::Command::new(editor)
                .arg(&full_path)
                .status()
                .expect("should be able to open file with editor");
            Ok(None)
        } else {
            open::that(&full_path).is_ok();
            Ok(None)
        }
    } else {
        Err((
            1,
            format!(
                "{} not found.  Would you like to add it to Mnemonic?",
                file_name.yellow().bold()
            ),
        ))
    }
}
fn append_to_mnemonic(file_path: &str, file_name: &str, text_to_append: &str) -> Option<String> {
    let mut mnemonic_file = fs::OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("guaranteed by caller");

    mnemonic_file
        .write_all(format!("\n{}", text_to_append).as_bytes())
        .expect("Should be able to write to mnemonic file");
    Some(format!(
        "'{}' added to {}",
        text_to_append,
        file_name.blue()
    ))
}

fn add(add_args: &ArgMatches, data_dir: &str) -> Result<Option<String>, (i32, String)> {
    let file_name = add_args.value_of("MNEMONIC").expect("Required by clap");
    let full_path = format!("{}/{}.md", data_dir, file_name);
    if !path::Path::new(&full_path).exists() {
        fs::File::create(&full_path).unwrap();
        if add_args.is_present("blank") {
            Ok(Some(format!("{} created.", file_name.blue())))
        } else {
            edit(add_args, &data_dir)
        }
    } else {
        Err((
            1,
            format!(
                "{} already exists.  Did you mean to edit it instead?",
                file_name.yellow().bold()
            ),
        ))
    }
}

fn rm(rm_args: &ArgMatches, data_dir: &str) -> Result<Option<String>, (i32, String)> {
    let file_name_arguments = rm_args.values_of("MNEMONIC").expect("required by clap");
    let mut output_msg = String::new();
    for file_name in file_name_arguments {
        let full_path = format!("{}/{}.md", data_dir, file_name);
        if !path::Path::new(&full_path).exists() {
            return Err((
                1,
                format!("No mnemonic named {} exists", file_name.yellow().bold()),
            ));
        }
        if rm_args.is_present("force") {
            let file_deleted_msg = delete_file(full_path, file_name)?;
            output_msg.push_str(format!("{}\n", file_deleted_msg.unwrap()).as_str());
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
                        let file_deleted_msg = delete_file(full_path, file_name)?;
                        output_msg.push_str(format!("{}\n", file_deleted_msg.unwrap()).as_str());
                        break;
                    }
                    "n\n" | "no\n" => break,
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
    }
    Ok(Some(output_msg))
}

fn delete_file(full_path: String, file_name: &str) -> Result<Option<String>, (i32, String)> {
    match fs::remove_file(full_path) {
        Err(e) => Err((
            1,
            format!(
                "There was an error deleting {}:\n{}",
                file_name.yellow().bold(),
                e
            ),
        )),
        _ => Ok(Some(format!("{} has been deleted.", file_name.blue()))),
    }
}

fn list(data_dir: &str) -> Result<Option<String>, (i32, String)> {
    let mut output_msg = String::new();
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

    output_msg.push_str(format!("Your {} available mnemonics are:\n", file_list.len()).as_str());
    file_list.sort();
    for line in file_list {
        output_msg.push_str(format!("{}\n", line).as_str());
    }
    Ok(Some(output_msg))
}

fn show(show_args: &ArgMatches, data_dir: &str) -> Result<Option<String>, (i32, String)> {
    use std::io::prelude::*;
    let usr_supplied_file_name = show_args.value_of("MNEMONIC").expect("Required by clap");
    let full_path = format!("{}/{}.md", &data_dir, usr_supplied_file_name);
    if show_args.is_present("plaintext") {
        let mut file = fs::File::open(&full_path).expect("should be able to open mnemonic");
        let mut plaintext = String::new();
        file.read_to_string(&mut plaintext)
            .expect("should be able to read open file to string");

        return Ok(Some(plaintext));
    }
    let theme = show_args.value_of("theme").unwrap_or("TwoDark");
    let syntax_language = show_args.value_of("syntax").unwrap_or("md");
    let printer = PrettyPrinter::default()
        .header(false)
        .grid(false)
        .language(syntax_language)
        .theme(theme)
        .line_numbers(false)
        .build()
        .expect("should be able to build a formater");

    match printer.file(full_path) {
        Ok(_) => Ok(None),
        Err(_) => Err((
            1,
            format!(
                "{} not found.  Would you like to add it to Mnemonic?",
                usr_supplied_file_name.yellow().bold()
            ),
        )),
    }
}
