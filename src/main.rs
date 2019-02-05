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
use list::list;
use rm::rm;
use show::show;
use state::State;
use toml;
use toml_edit::{value, Document};

fn main() {
    let cli_args = cli::build_cli().get_matches();

    // let toml = toml::to_string(&State::default()).unwrap();
    // // let mut doc = default_toml.parse::<Document>().expect("invalid doc");
    // doc["show"]["theme"] = value("OneHalfDark");
    // println!("{}", doc);
    let state = State::from_config_file()
        .and_from_clap_args(cli_args.clone())
        .and_from_filesystem();
    dbg!(&state);

    let result = match cli_args.subcommand_name() {
        Some("rm") => rm(state),
        Some("add") => add(state),
        Some("list") => list(state),
        // TODO: let user edit syntax for mnemonic
        Some("edit") => edit(state),
        Some("show") => show(state),
        _ => show(state),
    };
    // let config: State = toml::from_str(
    //     r#"
    //     mnemonics = ["notes"]

    //     [rm]
    //     force = true

    // "#,
    // )
    // .unwrap();

    // dbg!(&config);
    // let arg = State::from_test_state(TestStateBuilder::default().build().unwrap());

    // dbg!(&arg.show().syntax());

    match result {
        Ok(None) => (),
        Ok(Some(msg)) => println!("{}", msg),
        Err(cli_err) => cli_err.handle_err(),
    }
}
