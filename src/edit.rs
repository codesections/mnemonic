use crate::err::CliErr;
use crate::state::State;
use crate::utils;
use colored::*;
use std::{fs, io::Write};

pub fn edit(state: State) -> Result<Option<String>, CliErr> {
    let file_name = state.mnemonics()[0].clone();
    let full_path = format!("{}/{}.md", state.directory(), &file_name);

    if utils::new_mn_exists(&file_name, &state) {
        if let Some(text_to_append) = &state.edit().push() {
            Ok(append_to_mnemonic(&full_path, &file_name, &text_to_append))
        } else {
            #[cfg(not(test))]
            match std::process::Command::new(state.add().editor().clone())
                .arg(&full_path)
                .status()
            {
                Ok(_) => (),
                Err(_) => {
                    open::that(&full_path).expect("can open with xdg-open");
                }
            }
            Ok(None)
        }
    } else {
        Err(CliErr::MnemonicNotFound(file_name.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{test_state::*, *};
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn edit_non_existant_mn() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .build()
                .unwrap(),
        );

        match edit(state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file doesn't exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }

    #[test]
    fn edit_mn_with_editor() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        match edit(state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(true, "Should return Ok after opening file with $EDITOR"),
        }
    }

    #[test]
    fn edit_mn_with_xdg_open() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        match edit(state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(true, "Should return Ok after opening file with xdg-open"),
        }
    }

    #[test]
    fn append_to_a_mn() {
        use std::fs::File;
        use std::io::prelude::*;
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());

        let temp_file = temp_dir.child("mn0.md");
        temp_file.touch().unwrap();
        let temp_file_path = format!("{}", temp_file.path().display());

        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .edit(
                    EditBuilder::new()
                        .push(Some("text to append".to_string()))
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        match edit(state) {
            Err(e) => assert!(false, format!("No other errors: {:?}", e)),
            Ok(_) => assert!(true, "Should not return Ok if file does not exist"),
        }

        let mut file_contents = String::new();
        let mut temp_file = File::open(temp_file_path).unwrap();
        temp_file.read_to_string(&mut file_contents).unwrap();
        assert_eq!("\ntext to append", file_contents)
    }
}
