use colored::*;

#[derive(Debug)]
pub enum CliErr {
    MnemonicNotFound(String),
    ErrDeletingMnemonic(String, std::io::Error),
    MnemonicAlreadyExists(String),
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
        }
        std::process::exit(1)
    }
}
