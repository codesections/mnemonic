use crate::edit;
use crate::err::CliErr;
use crate::state::State;
use crate::utils;
use colored::*;
use std::fs;

pub fn add(state: State) -> Result<String, CliErr> {
    let file_name = &state.mnemonics()[0].clone();
    let full_path = format!("{}/{}.md", &state.directory(), file_name);
    if !utils::new_mn_exists(&file_name, &state) {
        fs::create_dir_all(&state.directory())?;
        fs::File::create(&full_path)?;
        let state = state.with_new_mnemonic_file(file_name.to_string());
        if *state.add().blank() {
            Ok(format!("{} created.", file_name.blue()))
        } else {
            edit(state)
        }
    } else {
        Err(CliErr::MnemonicAlreadyExists(file_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{test_state::*, *};
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn add_mn_that_exitst() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("already_exists.md").touch().expect("test");
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .add(
                    AddBuilder::new()
                        .blank(true)
                        .editor("nvim")
                        .build()
                        .expect("test"),
                )
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .expect("test"),
                )
                .build()
                .expect("test"),
        );

        match add(state) {
            Err(CliErr::MnemonicAlreadyExists(_)) => assert!(
                true,
                "Should error if attempting to add a file that already exists"
            ),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file already exists"),
        }
    }

    #[test]
    fn add_a_new_mn_with_editor() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .add(
                    AddBuilder::default()
                        .blank(true)
                        .editor("nvim")
                        .build()
                        .expect("test"),
                )
                .build()
                .expect("test"),
        );

        match add(state) {
            Err(e) => assert!(false, format!("No errors, such as: {:?}", e)),
            Ok(_) => assert!(true, "Should return Ok after creating a new file"),
        }
    }

    #[test]
    fn add_a_new_mn_with_xdg_open() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .add(
                    AddBuilder::default()
                        .blank(true)
                        .editor("foo")
                        .build()
                        .expect("test"),
                )
                .build()
                .expect("test"),
        );

        match add(state) {
            Err(e) => assert!(false, format!("No errors, such as: {:?}", e)),
            Ok(_) => assert!(true, "Should return Ok after creating a new file"),
        }
    }

    #[test]
    fn add_a_blank_mn() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .mnemonics(vec!["mn0".to_string()])
                .add(
                    AddBuilder::default()
                        .blank(true)
                        .editor("nvim")
                        .build()
                        .expect("test"),
                )
                .build()
                .expect("test"),
        );

        match add(state) {
            Err(e) => assert!(false, format!("No errors, such as: {:?}", e)),
            Ok(msg) => assert_eq!(msg, format!("{} created.", "mn0".blue())),
        }
    }
}
