use clap::ArgMatches;
use std::{env, ffi, fs, path};

pub struct FsState {
    pub editor: Option<ffi::OsString>,
    pub file_exists: bool,
    pub dir_contents: Option<fs::ReadDir>,
}

impl FsState {
    pub fn new() -> Self {
        Self {
            editor: None,
            file_exists: false,
            dir_contents: None,
        }
    }
    pub fn set_editor(self) -> Self {
        let editor = if let Some(editor) = env::var_os("VISUAL") {
            Some(editor)
        } else {
            env::var_os("EDITOR")
        };
        Self { editor, ..self }
    }

    pub fn set_file_exists(self, args_from_clap: &ArgMatches, data_dir: &str) -> Self {
        let file_name = args_from_clap
            .value_of("MNEMONIC")
            .expect("Required by clap");
        let full_path = format!("{}/{}.md", data_dir, file_name);
        Self {
            file_exists: path::Path::new(&full_path).exists(),
            ..self
        }
    }

    pub fn set_dir_contents(self, data_dir: &str) -> Self {
        let dir_contents =
            fs::read_dir(data_dir).expect("Should be able to read the local data directory");
        Self {
            dir_contents: Some(dir_contents),
            ..self
        }
    }
}
