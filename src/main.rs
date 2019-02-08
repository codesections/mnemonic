mod add;
mod cli;
mod edit;
mod err;
mod list;
mod rm;
mod show;
mod state;
mod utils;

use add::add;
use edit::edit;
use err::CliErr;
use list::list;
use rm::rm;
use show::show;
use state::State;

fn main() {
    match run() {
        Ok(std_out) => print!("{}", std_out),
        Err(cli_err) => cli_err.handle_err(),
    };
}

fn run() -> Result<String, CliErr> {
    let cli_args = cli::build_cli().get_matches();

    let state = State::from_config_file()?
        .and_from_clap_args(cli_args.clone())
        .and_from_filesystem()?;

    match cli_args.subcommand_name() {
        Some("rm") => rm(state),
        Some("add") => add(state),
        Some("list") => list(state),
        Some("edit") => edit(state),
        Some("show") => show(state),
        _ => show(state),
    }
}
