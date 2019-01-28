mod add;
mod cli;
mod edit;
mod err;
mod input_state;
mod list;
mod rm;
mod show;

use add::add;
use directories::ProjectDirs;
use edit::edit;
use input_state::FsState;
use list::list;
use rm::rm;
use show::show;
use std::fs;

fn main() {
    let cli_args = cli::build_cli().get_matches();
    let data_dir = ProjectDirs::from("", "", "mn")
        .expect("Should be able to determine project directory")
        .data_local_dir()
        .to_str()
        .expect("Should be able to find local data directory inside project directory")
        .to_string();

    fs::create_dir_all(&data_dir)
        .expect("should be able to create the data directory if it does not already exist");

    let result = match cli_args.subcommand() {
        ("rm", Some(rm_args)) => {
            let fs_state = FsState::new().set_file_exists(rm_args, &data_dir);
            rm(rm_args, &data_dir, fs_state)
        }
        ("add", Some(add_args)) => {
            let fs_state = FsState::new()
                .set_editor()
                .set_file_exists(add_args, &data_dir);
            add(add_args, &data_dir, fs_state)
        }
        ("list", Some(_list_args)) => {
            let fs_state = FsState::new().set_dir_contents(&data_dir);
            list(fs_state)
        }
        // TODO: let user edit syntax for mnemonic
        ("edit", Some(edit_args)) => {
            let fs_state = FsState::new()
                .set_editor()
                .set_file_exists(edit_args, &data_dir);
            edit(edit_args, &data_dir, fs_state)
        }
        ("show", Some(show_args)) => {
            let fs_state = FsState::new().set_file_exists(show_args, &data_dir);
            show(&show_args, &data_dir, fs_state)
        }
        _ => {
            let fs_state = FsState::new().set_file_exists(&cli_args, &data_dir);
            show(&cli_args, &data_dir, fs_state)
        }
    };

    match result {
        Ok(None) => (),
        Ok(Some(msg)) => println!("{}", msg),
        Err(cli_err) => cli_err.handle_err(),
    }
}
