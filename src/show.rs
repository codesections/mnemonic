use crate::err::CliErr;
use crate::state::State;
use crate::utils;
use prettyprint::*;
use std::{fs, io::Read};

pub fn show(state: State) -> Result<Option<String>, CliErr> {
    let mn = state.mnemonics()[0].clone();
    let directory = state.directory();
    let full_path = format!("{}/{}.md", directory, mn);

    if utils::new_mn_exists(&mn, &state) {
        if *state.show().plaintext() {
            return print_plaintext(&full_path);
        }
        print_color(&full_path, state)
    } else {
        Err(CliErr::MnemonicNotFound(mn.to_string()))
    }
}

fn print_plaintext(file_path: &str) -> Result<Option<String>, CliErr> {
    let mut file = fs::File::open(&file_path).expect("should be able to open mnemonic");
    let mut plaintext = String::new();
    file.read_to_string(&mut plaintext)
        .expect("should be able to read open file to string");
    Ok(Some(plaintext))
}

fn print_color(full_path: &str, state: State) -> Result<Option<String>, CliErr> {
    PrettyPrinter::default()
        .header(false)
        .grid(false)
        .language(state.show().syntax().as_str())
        .theme(state.show().theme().as_str())
        .line_numbers(false)
        .build()
        .expect("should be able to build a formater")
        .file(full_path)
        .expect("should be able to print existing file");
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::test_state::*;
    use crate::state::*;
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn show_invalid_mn() {
        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .build()
                .unwrap(),
        );

        match show(state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }

    #[test]
    fn show_no_flags() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();

        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        match show(state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(Some(msg)) => assert!(false, format!("Should not return a msg: {}", msg)),
            Ok(None) => assert!(true, "Should return Ok with no msg"),
        }
    }

    #[test]
    fn show_plaintext() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").write_str("test text").unwrap();

        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .show(ShowBuilder::new().plaintext(true).build().unwrap())
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        match show(state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(None) => assert!(false, "Should return Ok a no msg"),
            Ok(Some(msg)) => assert_eq!(msg, "test text"),
        }
    }

    #[test]
    fn print_color_no_args() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();

        let full_path = format!("{}/mn0.md", temp_dir_path);
        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        match print_color(&full_path, state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(None) => assert!(true, "Should return Ok with no msg"),
            Ok(Some(msg)) => assert!(false, "Should not return Some(msg): {}", msg),
        }
    }

    #[test]
    fn print_color_syntax() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();

        let full_path = format!("{}/mn0.md", temp_dir_path);
        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .show(
                    ShowBuilder::new()
                        .theme("TwoDark")
                        .syntax("md")
                        .build()
                        .unwrap(),
                )
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );
        match print_color(&full_path, state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(None) => assert!(true, "Should return Ok with no msg"),
            Ok(_) => assert!(false, "Should not return Some(msg)"),
        }
    }
    #[test]
    fn print_color_theme() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();

        let full_path = format!("{}/mn0.md", temp_dir_path);
        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .show(ShowBuilder::new().theme("OneHalfDark").build().unwrap())
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        match print_color(&full_path, state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(None) => assert!(true, "Should return Ok with no msg"),
            Ok(_) => assert!(false, "Should not return Some(msg)"),
        }
    }
}
