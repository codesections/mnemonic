use crate::err::CliErr;
use crate::input_state::{FsState, MnArgs};
use crate::utils;
use colored::*;
use std::{fs, io::Write};

pub fn edit(args: &MnArgs, fs_state: FsState) -> Result<Option<String>, CliErr> {
    let file_name = args.mn().as_ref().expect("Required by clap");
    let full_path = format!(
        "{}/{}.md",
        fs_state
            .data_dir()
            .as_ref()
            .expect("data_dir set by caller"),
        file_name
    );

    dbg!("ding");
    let editor = fs_state.editor().clone();
    if utils::mn_exists(file_name, &fs_state) {
        if let Some(text_to_append) = &args.push() {
            Ok(append_to_mnemonic(&full_path, &file_name, &text_to_append))
        } else if let Some(_editor) = editor {
            #[cfg(not(test))]
            std::process::Command::new(_editor)
                .arg(&full_path)
                .status()
                .expect("should be able to open file with editor");
            Ok(None)
        } else {
            #[cfg(not(test))]
            open::that(&full_path).is_ok();
            Ok(None)
        }
    } else {
        Err(CliErr::MnemonicNotFound(file_name.to_string()))
    }
}

fn append_to_mnemonic(file_path: &str, file_name: &str, text_to_append: &str) -> Option<String> {
    dbg!(&file_path);
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
    use crate::input_state::*;
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn it_returns_mn_not_found_err_when_asked_to_edit_invalid_mn() {
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn that doesn't exist"));
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let test_state = FsState::from_test_data(TestFsState::new().data_dir(&temp_dir_path));

        match edit(&args, test_state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file doesn't exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }

    #[test]
    fn it_can_edit_a_mn_with_editor() {
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0"));
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();
        let test_state = FsState::from_test_data(
            TestFsState::new()
                .editor("/usr/bin/nvim")
                .data_dir(&temp_dir_path)
                .mn_files(vec!["mn0.md".to_string()]),
        );

        match edit(&args, test_state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(true, "Should return Ok after opening file with $EDITOR"),
        }
    }

    #[test]
    fn it_can_edit_a_mn_with_xdg_open() {
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0"));
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();
        let test_state = FsState::from_test_data(
            TestFsState::new()
                .data_dir(&temp_dir_path)
                .mn_files(vec!["mn0.md".to_string()]),
        );

        match edit(&args, test_state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(true, "Should return Ok after opening file with xdg-open"),
        }
    }

    #[test]
    fn it_can_append_to_a_mn() {
        use std::fs::File;
        use std::io::prelude::*;
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0").push("text to append"));
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());

        let temp_file = temp_dir.child("mn0.md");
        temp_file.touch().unwrap();
        let temp_file_path = format!("{}", temp_file.path().display());

        let test_state = FsState::from_test_data(
            TestFsState::new()
                .data_dir(&temp_dir_path)
                .mn_files(vec![format!("{}.md", "mn0")]),
        );

        match edit(&args, test_state) {
            Err(e) => assert!(false, format!("No other errors: {:?}", e)),
            Ok(_) => assert!(true, "Should not return Ok if file does not exist"),
        }

        let mut file_contents = String::new();
        let mut temp_file = File::open(temp_file_path).unwrap();
        temp_file.read_to_string(&mut file_contents).unwrap();
        assert_eq!("\ntext to append", file_contents)
    }
}
