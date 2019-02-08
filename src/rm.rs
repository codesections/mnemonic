use crate::err::CliErr;
use crate::state::State;
use crate::utils;
use colored::*;
use std::{fs, io};

pub fn rm(state: State) -> Result<String, CliErr> {
    let file_name_arguments = state.mnemonics();
    let dir_path = state.directory();
    let mut output_msg = String::new();
    for file_name in file_name_arguments {
        let full_path = format!("{}/{}.md", dir_path, file_name);
        if !utils::new_mn_exists(file_name, &state) {
            return Err(CliErr::MnemonicNotFound(file_name.to_string()));
        }
        if *state.rm().force() {
            delete_file(full_path, &file_name)?;
            // the list of mn_files in fs_state isn't mutable from here and thus isn't updated
            output_msg.push_str(format!("{} has been deleted.", &file_name.blue()).as_str());
        } else {
            println!(
                "Are you sure you want to delete {}? [y/n]",
                file_name.yellow().bold()
            );
            let mut answer = String::new();
            // NOTE: state
            io::stdin().read_line(&mut answer)?;
            loop {
                match &answer[..] {
                    "y\n" | "yes\n" => {
                        delete_file(full_path, &file_name)?;
                        output_msg
                            .push_str(format!("{} has been deleted.", &file_name.blue()).as_str());
                        break;
                    }
                    "n\n" | "no\n" => break,
                    _ => {
                        println!("Please type 'yes' ('y') or 'no' ('n')");
                        answer = String::new();
                        io::stdin().read_line(&mut answer)?;
                    }
                }
            }
        }
    }
    Ok(output_msg)
}

fn delete_file(full_path: String, file_name: &str) -> Result<(), CliErr> {
    // NOTE: state
    match fs::remove_file(full_path) {
        Err(e) => Err(CliErr::ErrDeletingMnemonic(file_name.to_string(), e)),
        _ => Ok(()),
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
                .expect("test"),
        );

        match rm(state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }

    #[test]
    fn delete_a_file() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().expect("test");
        let full_path = format!("{}/mn0.md", temp_dir_path);

        match delete_file(full_path, "mn0") {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(()) => assert!(true, "should return Ok"),
        }
    }
    #[test]
    fn rm_force_flag() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().expect("test");

        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .rm(RmBuilder::new().force(true).build().expect("test"))
                .build()
                .expect("test"),
        )
        .and_from_filesystem()
        .expect("test");
        match rm(state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(msg) => assert!(true, "should return Ok with a msg: {}", msg),
        }
    }
}
