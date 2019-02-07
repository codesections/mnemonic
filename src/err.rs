use colored::*;

#[derive(Debug)]
pub enum CliErr {
    MnemonicNotFound(String),
    ErrDeletingMnemonic(String, std::io::Error),
    MnemonicAlreadyExists(String),
    InputOutput(std::io::Error),
    Toml(toml::de::Error),
    ParseEnv(std::ffi::OsString),
}

impl CliErr {
    pub fn handle_err(self) {
        use CliErr::*;
        match self {
            MnemonicNotFound(mnemonic) => eprintln!(
                "You do not have a mnemonic named {}.\nYou can add it with `mn add {}`",
                mnemonic.yellow().bold(),
                mnemonic,
            ),
            ErrDeletingMnemonic(mnemonic, err) => eprintln!(
                "There was an error deleting {}:\n{}",
                mnemonic.yellow().bold(),
                err
            ),
            MnemonicAlreadyExists(mnemonic) => eprintln!(
                "{} already exists.  Did you mean to edit it instead?",
                mnemonic.yellow().bold()
            ),
            InputOutput(err) => eprintln!("Error accessing file: {}", err),
            Toml(err) => eprintln!("Could not read your config file.  Please make sure all valid keys are present or reset to default.  {}", err),
            ParseEnv(err) => eprintln!("Could not read environmental variables.  Please ensure $VISUAL and $EDITOR are set to valid Unicode values: {:?}", err),
        }
        std::process::exit(1)
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

impl From<std::ffi::OsString> for CliErr {
    fn from(err: std::ffi::OsString) -> CliErr {
        CliErr::ParseEnv(err)
    }
}
