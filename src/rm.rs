use crate::err::CliErr;
use crate::input_state::FsState;
use clap::ArgMatches;
use colored::*;
use std::{fs, io};

pub fn rm(
    rm_args: &ArgMatches,
    data_dir: &str,
    fs_state: FsState,
) -> Result<Option<String>, CliErr> {
    let file_name_arguments = rm_args.values_of("MNEMONIC").expect("required by clap");
    let mut output_msg = String::new();
    for file_name in file_name_arguments {
        let full_path = format!("{}/{}.md", data_dir, file_name);
        if !fs_state.file_exists {
            return Err(CliErr::MnemonicNotFound(file_name.to_string()));
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
            // NOTE: state
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

fn delete_file(full_path: String, file_name: &str) -> Result<Option<String>, CliErr> {
    // NOTE: state
    match fs::remove_file(full_path) {
        Err(e) => Err(CliErr::ErrDeletingMnemonic(file_name.to_string(), e)),
        _ => Ok(Some(format!("{} has been deleted.", file_name.blue()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli;
    use crate::input_state::FsState;

    #[test]
    fn it_returns_mn_not_found_err_when_asked_to_rm_invalid_mn() {
        let test_state = FsState {
            file_exists: false,
            dir_contents: None,
            editor: None,
        };
        let args = cli::build_cli().get_matches_from(vec!["show", "nots"]);

        match rm(&args, "hi", test_state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }
}
