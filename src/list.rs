use crate::err::CliErr;
use crate::state::State;
use colored::*;

pub fn list(state: State) -> Result<String, CliErr> {
    let mut output_msg = String::new();
    let mut file_list = vec![];
    for file in state.filesystem().mnemonic_files() {
        file_list.push(format!("  - {}", file.as_str().blue().bold()));
    }

    match file_list.len() {
        0 => return Ok("You don't have any mnemonics yet.  Use `mn add <MNEMONIC>` to create your first mnemonic.".to_string()),
        1 => output_msg.push_str("Your 1 available mnemonic is:\n"),
        _ => output_msg.push_str(format!("Your {} available mnemonics are:\n", file_list.len()).as_str()),
    }

    file_list.sort();
    for line in file_list {
        output_msg.push_str(format!("{}\n", line).as_str());
    }
    Ok(output_msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{test_state::*, *};
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn list_zero_mns() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());

        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .build()
                .expect("test"),
        )
        .and_from_filesystem()
        .expect("test");
        match list(state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(msg) => assert_eq!(
                msg,
                String::from("You don't have any mnemonics yet.  Use `mn add <MNEMONIC>` to create your first mnemonic.")
            ),
        }
    }

    #[test]
    fn list_one_mn() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());

        temp_dir.child("mn0.md").touch().expect("test");
        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .build()
                .expect("test"),
        )
        .and_from_filesystem()
        .expect("test");

        match list(state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(msg) => assert_eq!(
                msg,
                format!(
                    "Your 1 available mnemonic is:\n  - {}\n",
                    "mn0".blue().bold()
                )
            ),
        }
    }

    #[test]
    fn list_multiple_mns() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());

        temp_dir.child("mn0.md").touch().expect("test");
        temp_dir.child("mn1.md").touch().expect("test");
        temp_dir.child("mn2.md").touch().expect("test");

        let state = State::from_test_state(
            TestStateBuilder::new()
                .directory(temp_dir_path)
                .build()
                .expect("test"),
        )
        .and_from_filesystem()
        .expect("test");
        match list(state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(msg) => assert_eq!(
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
