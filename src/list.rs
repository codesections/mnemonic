use crate::err::CliErr;
use crate::input_state::FsState;
use colored::*;

pub fn list(fs_state: FsState) -> Result<Option<String>, CliErr> {
    let mut output_msg = String::new();
    let mut file_list = vec![];
    for file in fs_state.dir_contents().expect("dir_contents set by caller") {
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

    match file_list.len() {
        0 => return Ok(Some("You don't have any mnemonics yet.  Use `mn add <MNEMONIC>` to create your first mnemonic.".to_string())),
        1 => output_msg.push_str("Your 1 available mnemonic is:\n"),
        _ => output_msg.push_str(format!("Your {} available mnemonics are:\n", file_list.len()).as_str()),
    }

    file_list.sort();
    for line in file_list {
        output_msg.push_str(format!("{}\n", line).as_str());
    }
    Ok(Some(output_msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_state::*;
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn it_returns_an_appropriate_msg_when_the_user_has_no_mn() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());

        let test_state = FsState::from_test_data(TestFsState::new().dir_contents(&temp_dir_path));
        match list(test_state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(None) => assert!(false),
            Ok(Some(msg)) => assert_eq!(
                msg,
                String::from("You don't have any mnemonics yet.  Use `mn add <MNEMONIC>` to create your first mnemonic.")
            ),
        }
    }

    #[test]
    fn it_returns_an_appropriate_msg_when_the_user_has_one_mn() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());

        let test_state = FsState::from_test_data(TestFsState::new().dir_contents(&temp_dir_path));

        temp_dir.child("mn0.md").touch().unwrap();

        match list(test_state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(None) => assert!(false),
            Ok(Some(msg)) => assert_eq!(
                msg,
                format!(
                    "Your 1 available mnemonic is:\n  - {}\n",
                    "mn0".blue().bold()
                )
            ),
        }
    }

    #[test]
    fn it_returns_an_appropriate_msg_when_the_user_has_many_mns() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());

        let test_state = FsState::from_test_data(TestFsState::new().dir_contents(&temp_dir_path));

        temp_dir.child("mn0.md").touch().unwrap();
        temp_dir.child("mn1.md").touch().unwrap();
        temp_dir.child("mn2.md").touch().unwrap();

        match list(test_state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(None) => assert!(false),
            Ok(Some(msg)) => assert_eq!(
                msg,
                format!(
                    "Your 3 available mnemonics are:\n  - {}\n  - {}\n  - {}\n",
                    "mn0".blue().bold(),
                    "mn1".blue().bold(),
                    "mn2".blue().bold(),
                )
            ),
        }
    }
}
