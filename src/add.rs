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
