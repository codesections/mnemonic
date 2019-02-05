use crate::err::CliErr;
use crate::state::State;
use crate::utils;
use colored::*;
use std::{fs, io};

pub fn rm(state: State) -> Result<Option<String>, CliErr> {
    let file_name_arguments = state.mnemonics();
    let dir_path = state.directory();
    let mut output_msg = String::new();
    for file_name in file_name_arguments {
        let full_path = format!("{}/{}.md", dir_path, file_name);
        if !utils::new_mn_exists(file_name, &state) {
            return Err(CliErr::MnemonicNotFound(file_name.to_string()));
        }
        if *state.rm().force() {
            let file_deleted_msg = delete_file(full_path, &file_name)?;
            // the list of mn_files in fs_state isn't mutable from here and thus isn't updated
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
                        let file_deleted_msg = delete_file(full_path, &file_name)?;
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
    use crate::state::{test_state::*, *};
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn rm_invalid_file() {
        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .build()
                .unwrap(),
        );

        match rm(state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }

    #[test]
    fn delete_a_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();
        let full_path = format!("{}/mn0.md", temp_dir_path);

        match delete_file(full_path, "mn0") {
            Ok(None) => assert!(false, "should print a msg"),
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(Some(_)) => assert!(true, "should return Some(msg)"),
        }
    }
    #[test]
    fn rm_force_flag() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();

        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .rm(RmBuilder::new().force(true).build().unwrap())
                .build()
                .unwrap(),
        )
        .and_from_filesystem();
        match rm(state) {
            Ok(None) => assert!(false, "should print a msg"),
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(Some(_)) => assert!(true, "should return Some(msg)"),
        }
    }
}
