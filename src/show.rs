use crate::err::CliErr;
use crate::input_state::FsState;
use clap::ArgMatches;
use prettyprint::*;
use std::{fs, io::Read};

pub fn show(
    show_args: &ArgMatches,
    data_dir: &str,
    fs_state: FsState,
) -> Result<Option<String>, CliErr> {
    let usr_supplied_file_name = show_args.value_of("MNEMONIC").expect("Required by clap");
    let full_path = format!("{}/{}.md", &data_dir, usr_supplied_file_name);
    if fs_state.file_exists {
        if show_args.is_present("plaintext") {
            let mut file = fs::File::open(&full_path).expect("should be able to open mnemonic");
            let mut plaintext = String::new();
            file.read_to_string(&mut plaintext)
                .expect("should be able to read open file to string");
            return Ok(Some(plaintext));
        }
        let theme = show_args.value_of("theme").unwrap_or("TwoDark");
        let syntax_language = show_args.value_of("syntax").unwrap_or("md");
        PrettyPrinter::default()
            .header(false)
            .grid(false)
            .language(syntax_language)
            .theme(theme)
            .line_numbers(false)
            .build()
            .expect("should be able to build a formater")
            .file(full_path)
            .expect("should be able to print existing file");
        Ok(None)
    } else {
        Err(CliErr::MnemonicNotFound(usr_supplied_file_name.to_string()))
    }
}
