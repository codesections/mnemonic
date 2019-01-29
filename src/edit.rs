use crate::err::CliErr;
use crate::input_state::FsState;
use clap::ArgMatches;
use colored::*;
use std::{fs, io::Write, process};

pub fn edit(
    edit_args: &ArgMatches,
    data_dir: &str,
    fs_state: FsState,
) -> Result<Option<String>, CliErr> {
    let file_name = edit_args.value_of("MNEMONIC").expect("Required by clap");
    let full_path = format!("{}/{}.md", data_dir, file_name);

    let editor = fs_state.editor;
    if fs_state.file_exists {
        if let Some(text_to_append) = edit_args.value_of("push") {
            Ok(append_to_mnemonic(&full_path, &file_name, text_to_append))
        } else if let Some(editor) = editor {
            process::Command::new(editor)
                .arg(&full_path)
                .status()
                .expect("should be able to open file with editor");
            Ok(None)
        } else {
            open::that(&full_path).is_ok();
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
    use crate::cli;
    use crate::input_state::FsState;
    #[test]
    fn it_returns_mn_not_found_err_when_asked_to_edit_invalid_mn() {
        let test_state = FsState {
            file_exists: false,
            dir_contents: None,
            editor: None,
        };
        let args = cli::build_cli().get_matches_from(vec!["show", "nots"]);

        match edit(&args, "non_existent_dir", test_state) {
            Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
            Err(_) => assert!(false, "No other errors"),
            Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
        }
    }
}
