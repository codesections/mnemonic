use crate::err::CliErr;
use crate::input_state::FsState;
use colored::*;

pub fn list(fs_state: FsState) -> Result<Option<String>, CliErr> {
    let mut output_msg = String::new();
    let mut file_list = vec![];
    for file in fs_state.dir_contents.expect("Set by caller") {
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

    output_msg.push_str(format!("Your {} available mnemonics are:\n", file_list.len()).as_str());
    file_list.sort();
    for line in file_list {
        output_msg.push_str(format!("{}\n", line).as_str());
    }
    Ok(Some(output_msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_state::FsState;

    #[test]
    fn it_lists_the_files_in_the_current_dir() {
        let test_dir = std::fs::read_dir(".");
        let test_state = FsState {
            file_exists: false,
            dir_contents: Some(test_dir.unwrap()),
            editor: None,
        };

        match list(test_state) {
            Err(_) => assert!(false, "No other errors"),
            Ok(None) => assert!(false),
            Ok(Some(_msg)) => assert!(true, "It lists the files in the current dir"),
        }
    }
}
