use crate::err::CliErr;
use crate::input_state::FsState;
use clap::ArgMatches;
use prettyprint::*;
use std::{fs, io::Read};

pub fn show(
    show_args: &ArgMatches,
    data_dir: &str,
    fs_state: FsState,
) -> Result<Option<String>, CliErr> {
    let usr_supplied_file_name = show_args.value_of("MNEMONIC").expect("Required by clap");
    let full_path = format!("{}/{}.md", &data_dir, usr_supplied_file_name);
    if fs_state.file_exists {
        if show_args.is_present("plaintext") {
            return print_plaintext(&full_path);
        }
        print_color(&full_path, show_args)
    } else {
        Err(CliErr::MnemonicNotFound(usr_supplied_file_name.to_string()))
    }
}

fn print_plaintext(file_path: &String) -> Result<Option<String>, CliErr> {
    // not unit tested
    let mut file = fs::File::open(&file_path).expect("should be able to open mnemonic");
    let mut plaintext = String::new();
    file.read_to_string(&mut plaintext)
        .expect("should be able to read open file to string");
    Ok(Some(plaintext))
}

fn print_color(full_path: &String, show_args: &ArgMatches) -> Result<Option<String>, CliErr> {
    // not unit tested (depends on file-system contents)
    let theme = show_args.value_of("theme").unwrap_or("TwoDark");
    let syntax_language = show_args.value_of("syntax").unwrap_or("md");
    PrettyPrinter::default()
        .header(false)
        .grid(false)
        .language(syntax_language)
        .theme(theme)
        .line_numbers(false)
        .build()
        .expect("should be able to build a formater")
        .file(full_path.as_str())
        .expect("should be able to print existing file");
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli;
    use crate::input_state::FsState;
    #[test]
    fn it_returns_mn_not_found_err_when_asked_to_show_invalid_mn() {
        let test_state = FsState {
            file_exists: false,
            dir_contents: None,
            editor: None,
        };
        let args = cli::build_cli().get_matches_from(vec!["show", "nots"]);

        match show(&args, "hi", test_state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }
}
