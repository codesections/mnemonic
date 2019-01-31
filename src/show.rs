use crate::err::CliErr;
use crate::input_state::FsState;
use crate::utils;
use crate::MnArgs;
use prettyprint::*;
use std::{fs, io::Read};

pub fn show(args: &MnArgs, fs_state: FsState) -> Result<Option<String>, CliErr> {
    let mn = args.mn().as_ref().expect("Required by clap");
    let dir_path = fs_state
        .data_dir()
        .as_ref()
        .expect("data_dir set by caller");
    let full_path = format!("{}/{}.md", dir_path, mn);

    if utils::mn_exists(mn, &fs_state) {
        if *args.plaintext_flag() {
            return print_plaintext(&full_path);
        }
        print_color(&full_path, args)
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

fn print_color(full_path: &str, args: &MnArgs) -> Result<Option<String>, CliErr> {
    PrettyPrinter::default()
        .header(false)
        .grid(false)
        .language(args.syntax().clone().unwrap_or_else(|| "md".to_string()))
        .theme(
            args.theme()
                .clone()
                .unwrap_or_else(|| "TwoDark".to_string()),
        )
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
    use crate::input_state::MnArgs;
    use crate::input_state::{FsState, TestFsState, TestMnArgs};
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn show_invalid_mn() {
        let test_state =
            FsState::from_test_data(TestFsState::new().data_dir("dir that doesn't exist"));

        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn that doesn't exist"));

        match show(&args, test_state) {
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

        let test_state = FsState::from_test_data(
            TestFsState::new()
                .mn_files(vec!["mn0.md".to_string()])
                .data_dir(&temp_dir_path),
        );
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0"));

        match show(&args, test_state) {
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

        let test_state = FsState::from_test_data(
            TestFsState::new()
                .mn_files(vec!["mn0.md".to_string()])
                .data_dir(&temp_dir_path),
        );
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0").plaintext_flag(true));

        match show(&args, test_state) {
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
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0"));

        match print_color(&full_path, &args) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(None) => assert!(true, "Should return Ok with no msg"),
            Ok(_) => assert!(false, "Should not return Some(msg)"),
        }
    }

    #[test]
    fn print_color_syntax() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = format!("{}", temp_dir.path().display());
        temp_dir.child("mn0.md").touch().unwrap();

        let full_path = format!("{}/mn0.md", temp_dir_path);
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0").syntax("yaml"));

        match print_color(&full_path, &args) {
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
        let args = MnArgs::from_test_data(TestMnArgs::new().mn("mn0").theme("OneHalfDark"));

        match print_color(&full_path, &args) {
            Err(e) => assert!(false, format!("Should not have error: {:#?}", e)),
            Ok(None) => assert!(true, "Should return Ok with no msg"),
            Ok(_) => assert!(false, "Should not return Some(msg)"),
        }
    }
}
