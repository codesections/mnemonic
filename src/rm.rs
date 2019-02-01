use crate::err::CliErr;
use crate::input_state::FsState;
use crate::utils;
use crate::MnArgs;
use colored::*;
use std::{fs, io};

pub fn rm(args: &MnArgs, fs_state: FsState) -> Result<Option<String>, CliErr> {
    let file_name_arguments = args
        .mnemonics()
        .clone()
        .expect("mnemonics required by clap");
    let dir_path = fs_state
        .data_dir()
        .as_ref()
        .expect("data_dir set by caller");
    let mut output_msg = String::new();
    for file_name in file_name_arguments {
        let full_path = format!("{}/{}.md", dir_path, file_name);
        if !utils::mn_exists(&file_name, &fs_state) {
            return Err(CliErr::MnemonicNotFound(file_name.to_string()));
        }
        if *args.force_flag() {
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
    use crate::input_state::MnArgs;
    use crate::input_state::{FsState, TestFsState, TestMnArgs};
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn rm_invalid_file() {
        let test_state = FsState::from_test_data(TestFsState::new().data_dir("invalid"));
        let args =
            MnArgs::from_test_data(TestMnArgs::new().mnemonics(vec!["mn that doesn't exist"]));

        match rm(&args, test_state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }

    #[test]
    fn delte_a_file() {
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

        let test_state = FsState::from_test_data(
            TestFsState::new()
                .data_dir(&temp_dir_path)
                .mn_files(vec!["mn0.md".to_string()]),
        );

        let args =
            MnArgs::from_test_data(TestMnArgs::new().mnemonics(vec!["mn0"]).force_flag(true));

        match rm(&args, test_state) {
            Ok(None) => assert!(false, "should print a msg"),
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(Some(_)) => assert!(true, "should return Some(msg)"),
        }
    }
}
