use crate::err::CliErr;
use crate::state::State;
use crate::utils;
use prettyprint::*;
use std::{fs, io::Read};

pub fn show(state: State) -> Result<String, CliErr> {
    let mn = state.mnemonics()[0].clone();
    let directory = state.directory();
    let full_path = format!("{}/{}.md", directory, mn);

    if utils::new_mn_exists(&mn, &state) {
        if *state.show().plaintext() {
            return print_plaintext(&full_path);
        }
        print_color(&full_path, state)?;
        Ok(String::new())
    } else {
        Err(CliErr::MnemonicNotFound(mn.to_string()))
    }
}

fn print_plaintext(file_path: &str) -> Result<String, CliErr> {
    let mut file = fs::File::open(&file_path)?;
    let mut plaintext = String::new();
    file.read_to_string(&mut plaintext)?;
    Ok(plaintext)
}

fn print_color(full_path: &str, state: State) -> Result<(), CliErr> {
    PrettyPrinter::default()
        .header(false)
        .grid(false)
        .language(state.show().syntax().as_str())
        .theme(state.show().theme().as_str())
        .line_numbers(false)
        .build()
        .map_err(CliErr::CannotPrettyPrint)?
        .file(full_path)
        .map_err(|e| CliErr::CannotPrettyPrint(e.description().to_string()))?;
    Ok(())
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
                .expect("test"),
        );

        match show(state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }

    #[test]
    fn show_no_flags() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().expect("test");

        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .expect("test"),
                )
                .build()
                .expect("test"),
        );

        match show(state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(ref msg) if msg == &String::new() => assert!(true, "Should return Ok with no msg"),
            Ok(msg) => assert!(false, format!("Should not return a msg: {}", msg)),
        }
    }

    #[test]
    fn show_plaintext() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir
            .child("mn0.md")
            .write_str("test text")
            .expect("test");

        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .show(ShowBuilder::new().plaintext(true).build().expect("test"))
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .expect("test"),
                )
                .build()
                .expect("test"),
        );

        match show(state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(ref msg) if msg == &String::new() => assert!(false, "Should return Ok with a msg"),
            Ok(msg) => assert_eq!(msg, "test text"),
        }
    }

    #[test]
    fn print_color_no_args() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().expect("test");

        let full_path = format!("{}/mn0.md", temp_dir_path);
        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .filesystem(
                    FileSystemBuilder::new()
                        .mnemonic_files(vec!["mn0".to_string()])
                        .build()
                        .expect("test"),
                )
                .build()
                .expect("test"),
        );

        match print_color(&full_path, state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(()) => assert!(true, "Should return Ok(())"),
        }
    }

    #[test]
    fn print_color_syntax() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().expect("test");

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
        match print_color(&full_path, state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(()) => assert!(true, "Should return Ok with no msg"),
        }
    }
    #[test]
    fn print_color_theme() {
        let temp_dir = TempDir::new().expect("test");
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().expect("test");

        let full_path = format!("{}/mn0.md", temp_dir_path);
        let state = State::from_test_state(
            TestStateBuilder::new()
                .mnemonics(vec!["mn0".to_string()])
                .directory(temp_dir_path)
                .show(
                    ShowBuilder::new()
                        .theme("OneHalfDark")
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

        match print_color(&full_path, state) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(()) => assert!(true, "Should return Ok with no msg"),
        }
    }
}
