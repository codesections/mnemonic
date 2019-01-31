mod add;
mod cli;
mod edit;
mod err;
mod input_state;
mod list;
mod rm;
mod show;
mod utils;

use add::add;
use edit::edit;
use input_state::{FsState, MnArgs};
use list::list;
use rm::rm;
use show::show;

fn main() {
    let cli_args = cli::build_cli().get_matches();
    let fs_state = FsState::from_filesystem();

    let result = match cli_args.subcommand() {
        ("rm", Some(args)) => {
            let args = MnArgs::build_from_clap_args(args);
            rm(&args, fs_state)
        }
        ("add", Some(args)) => {
            let args = MnArgs::build_from_clap_args(args);
            add(&args, fs_state)
        }
        ("list", Some(_list_args)) => list(fs_state),
        // TODO: let user edit syntax for mnemonic
        ("edit", Some(args)) => {
            let args = MnArgs::build_from_clap_args(args);
            edit(&args, fs_state)
        }
        ("show", Some(args)) => {
            let args = MnArgs::build_from_clap_args(args);
            show(&args, fs_state)
        }
        _ => {
            let args = MnArgs::build_from_clap_args(&cli_args);
            show(&args, fs_state)
        }
    };

    match result {
        Ok(None) => (),
        Ok(Some(msg)) => println!("{}", msg),
        Err(cli_err) => cli_err.handle_err(),
    }
}
