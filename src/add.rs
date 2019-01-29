use crate::edit;
use crate::err::CliErr;
use crate::FsState;
use clap::ArgMatches;
use colored::*;
use std::fs;

pub fn add(
    add_args: &ArgMatches,
    data_dir: &str,
    fs_state: FsState,
) -> Result<Option<String>, CliErr> {
    let file_name = add_args.value_of("MNEMONIC").expect("Required by clap");
    let full_path = format!("{}/{}.md", data_dir, file_name);
    if !fs_state.file_exists {
        fs::File::create(&full_path).expect("Can create a file in the project dir");
        if add_args.is_present("blank") {
            Ok(Some(format!("{} created.", file_name.blue())))
        } else {
            edit(add_args, &data_dir, fs_state)
        }
    } else {
        Err(CliErr::MnemonicAlreadyExists(file_name.to_string()))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::cli;
//     use crate::input_state::FsState;
//     #[test]
//     fn it_returns_mn_not_found_err_when_asked_to_edit_invalid_mn() {
//         let test_state = FsState {
//             file_exists: false,
//             dir_contents: None,
//             editor: None,
//         };
//         let args = cli::build_cli().get_matches_from(vec!["show", "nots"]);

//         match add(&args, "non_existent_dir", test_state) {
//             Err(CliErr::MnemonicNotFound(_)) => assert!(true, "Should error if file does not exit"),
//             Err(_) => assert!(false, "No other errors"),
//             Ok(_) => assert!(false, "Should not return Ok if file does not exist"),
//         }
//     }
// }
