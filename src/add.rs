use crate::edit;
use crate::err::CliErr;
use crate::utils;
use crate::FsState;
use crate::MnArgs;
use colored::*;
use std::fs;

pub fn add(args: &MnArgs, fs_state: FsState) -> Result<Option<String>, CliErr> {
    let file_name = args.mn().as_ref().expect("Required by clap");
    let full_path = format!(
        "{}/{}.md",
        &fs_state.data_dir().as_ref().expect("dir_dir set by caller"),
        file_name
    );
    if !utils::mn_exists(file_name, &fs_state) {
        fs::File::create(&full_path).expect("Can create a file in the project dir");
        let fs_state = fs_state.add_mn_file(format!("{}.md", file_name));
        if *args.blank_flag() {
            Ok(Some(format!("{} created.", file_name.blue())))
        } else {
            edit(args, fs_state)
        }
    } else {
        Err(CliErr::MnemonicAlreadyExists(file_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_state::*;
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn add_mn_that_exitst() {
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("already_exists"));

        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("already_exists.md").touch().unwrap();
        let test_state = FsState::from_test_data(
            TestFsState::new()
                .data_dir(&temp_dir_path)
                .mn_files(vec!["already_exists.md".to_string()]),
        );

        match add(&args, test_state) {
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
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("new"));

        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let test_state = FsState::from_test_data(
            TestFsState::new()
                .editor("/usr/bin/nvim")
                .data_dir(&temp_dir_path),
        );

        match add(&args, test_state) {
            Err(e) => assert!(false, format!("No errors, such as: {:?}", e)),
            Ok(None) => assert!(true, "Should return Ok after creating a new file"),
            Ok(_) => assert!(false, format!("Should not return a success msg")),
        }
    }

    #[test]
    fn add_a_new_mn_with_xdg_open() {
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("new"));

        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let test_state = FsState::from_test_data(TestFsState::new().data_dir(&temp_dir_path));

        match add(&args, test_state) {
            Err(e) => assert!(false, format!("No errors, such as: {:?}", e)),
            Ok(None) => assert!(true, "Should return Ok after creating a new file"),
            Ok(_) => assert!(false, format!("Should not return a success msg")),
        }
    }

    #[test]
    fn add_a_blank_mn() {
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("new").blank_flag(true));

        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        let test_state = FsState::from_test_data(
            TestFsState::new()
                .data_dir(&temp_dir_path)
                .editor("/usr/bin/nvim"),
        );

        match add(&args, test_state) {
            Err(e) => assert!(false, format!("No errors, such as: {:?}", e)),
            Ok(msg) => assert_eq!(msg, Some(format!("{} created.", "new".blue()))),
        }
    }
}
