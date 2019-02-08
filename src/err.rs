#[derive(Debug)]
pub enum CliErr {
    MnemonicNotFound(String),
    ErrDeletingMnemonic(String, std::io::Error),
    MnemonicAlreadyExists(String),
    InputOutput(std::io::Error),
    Toml(toml::de::Error),
    TomlEdit(toml_edit::TomlError),
    ParseUnicode(String),
    CannotPrettyPrint(String),
    LocateDirs,
    #[allow(dead_code)]
    CannotGenerateManPage(String, std::io::Error),
}

impl CliErr {
    pub fn handle_err(self) {
        use colored::*;
        use CliErr::*;
        let (err_msg, err_code) = match self {
            MnemonicNotFound(mnemonic) => (format!(
                "You do not have a mnemonic named {}.\nYou can add it with `mn add {}`",
                mnemonic.yellow().bold(),
                mnemonic,
            ), 1),
            ErrDeletingMnemonic(mnemonic, err) => (format!(
                "There was an error deleting {}:\n{}",
                mnemonic.yellow().bold(),
                err
            ), 2),
            MnemonicAlreadyExists(mnemonic) => (format!(
                "{} already exists.  Did you mean to edit it instead?",
                mnemonic.yellow().bold()
            ), 3),
            InputOutput(err) => (format!(
                "Error accessing file: {}",
                err
            ), 4),
            Toml(err) => (format!(
                "Could not read your config file due to: {}.\nPlease make sure all valid keys are present or reset to default.",
                err.to_string().yellow().bold()
            ), 5),
            TomlEdit(err) => (format!(
                "Could not read your config file due to: {}.\nPlease make sure all valid keys are present or reset to default.",
                err.to_string().yellow().bold()
            ), 5),
            ParseUnicode(unreadable_os_string) => (format!(
                "Could not read {}",
                unreadable_os_string,
            ), 6),
            CannotPrettyPrint(err)=> (format!(
                "Could not format mnemonic for printing\n{}",
                err,
            ), 6),
            LocateDirs => (
                "Could not determine the proper directory to store files.  Please select a directory in your configuration file and try again.".to_string(),
            7),
            CannotGenerateManPage(path_to_man_page, err) => (format!(
                "Could not generate the manual page at {}: {}", path_to_man_page, err
            ), 8),

        };
        eprintln!("{}", err_msg);
        std::process::exit(err_code)
    }
}

use std::io;
impl From<io::Error> for CliErr {
    fn from(err: io::Error) -> CliErr {
        CliErr::InputOutput(err)
    }
}
impl From<toml::de::Error> for CliErr {
    fn from(err: toml::de::Error) -> CliErr {
        CliErr::Toml(err)
    }
}
impl From<toml_edit::TomlError> for CliErr {
    fn from(err: toml_edit::TomlError) -> CliErr {
        CliErr::TomlEdit(err)
    }
}
