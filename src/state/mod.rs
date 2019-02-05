mod default_toml_config;
pub mod test_state;
use clap::ArgMatches;
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::prelude::*};
use test_state::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    mnemonics: Vec<String>,
    directory: String,
    add: Add,
    edit: Edit,
    list: List,
    rm: Rm,
    show: Show,
    filesystem: FileSystem,
}

impl State {
    // getters
    pub fn add(&self) -> &Add {
        &self.add
    }
    pub fn edit(&self) -> &Edit {
        &self.edit
    }
    pub fn show(&self) -> &Show {
        &self.show
    }
    pub fn rm(&self) -> &Rm {
        &self.rm
    }
    pub fn mnemonics(&self) -> &Vec<String> {
        &self.mnemonics
    }
    pub fn directory(&self) -> &String {
        &self.directory
    }
    pub fn filesystem(&self) -> &FileSystem {
        &self.filesystem
    }

    // Constructors/setters
    pub fn from_test_state(test_state: TestState) -> Self {
        Self {
            mnemonics: test_state.mnemonics,
            directory: test_state.directory,
            add: test_state.add,
            edit: test_state.edit,
            list: test_state.list,
            rm: test_state.rm,
            show: test_state.show,
            filesystem: test_state.filesystem,
        }
    }

    pub fn from_config_file() -> Self {
        use default_toml_config::TOML;
        use directories::ProjectDirs;
        use std::{fs, io::Read};
        use toml_edit::{value, Document};

        let config_dir = ProjectDirs::from("", "", "mn")
            .expect("Should be able to determine project directory")
            .config_dir()
            .to_str()
            .expect("Should be able to locate config file")
            .to_string();

        let config_file = format!("{}/mn_config.toml", config_dir).to_string();

        let mut config = String::new();

        let state: State = match fs::File::open(&config_file) {
            Ok(mut file) => {
                file.read_to_string(&mut config)
                    .expect("should be able to read open file to string");
                // FIXME: add error handling here
                toml::from_str(config.as_str()).unwrap()
            }
            Err(_) => {
                use std::env;

                config = TOML.to_string();
                let mut state: State = toml::from_str(config.as_str()).unwrap();
                let directory = ProjectDirs::from("", "", "mn")
                    .expect("Should be able to determine project directory")
                    .data_local_dir()
                    .to_str()
                    .expect("Should be able to find local data directory inside project directory")
                    .to_string();
                let default_editor = if let Some(editor) = env::var_os("VISUAL") {
                    editor.into_string().expect("can parse $VISUAL")
                } else if let Some(editor) = env::var_os("EDITOR") {
                    editor.into_string().expect("Can parse $EDITOR")
                } else {
                    "nano".to_string()
                };
                state.edit.editor = default_editor.clone();
                state.add.editor = default_editor.clone();
                state.directory = directory.clone();
                let mut state_with_comments = TOML.parse::<Document>().expect("invalid doc");
                state_with_comments["edit"]["editor"] = value(default_editor.clone());
                state_with_comments["add"]["editor"] = value(default_editor);
                state_with_comments["directory"] = value(directory);
                let mut file = fs::File::create(&config_file).unwrap();
                file.write_all(state_with_comments.to_string().as_bytes())
                    .unwrap();

                state
            }
        };

        state
    }

    pub fn and_from_clap_args(self, clap_args: ArgMatches) -> Self {
        Self {
            mnemonics: match clap_args.values_of("MNEMONIC") {
                Some(clap_vec) => clap_vec.map(|s| s.to_string()).collect(),
                None => match clap_args.subcommand() {
                    ("add", Some(sub_args))
                    | ("edit", Some(sub_args))
                    | ("rm", Some(sub_args))
                    | ("show", Some(sub_args)) => match sub_args.values_of("MNEMONIC") {
                        Some(clap_vec) => clap_vec.map(|s| s.to_string()).collect(),
                        None => self.mnemonics,
                    },
                    (_, _) => self.mnemonics,
                },
            },
            add: Add {
                blank: match clap_args.subcommand_matches("add") {
                    Some(add_args) => add_args.is_present("blank") || self.add.blank,
                    None => self.add.blank,
                },
                editor: match clap_args.subcommand_matches("add") {
                    Some(add_args) => match add_args.value_of("editor") {
                        Some(editor) => editor.to_string(),
                        None => self.add.editor,
                    },
                    None => self.add.editor,
                },
            },
            rm: Rm {
                force: match clap_args.subcommand_matches("rm") {
                    Some(rm_args) => rm_args.is_present("force") || self.rm.force,
                    None => self.rm.force,
                },
            },
            show: Show {
                plaintext: match clap_args.subcommand_matches("show") {
                    Some(show_args) => show_args.is_present("plaintext") || self.show.plaintext,
                    None => self.show.plaintext,
                },
                theme: match clap_args.subcommand_matches("show") {
                    Some(show_args) => match show_args.value_of("theme") {
                        Some(theme) => theme.to_string(),
                        None => self.show.theme,
                    },
                    None => self.show.theme,
                },
                syntax: match clap_args.subcommand_matches("show") {
                    Some(show_args) => match show_args.value_of("syntax") {
                        Some(syntax) => syntax.to_string(),
                        None => self.show.syntax,
                    },
                    None => self.show.syntax,
                },
            },
            ..self
        }
    }

    pub fn and_from_filesystem(self) -> Self {
        fs::create_dir_all(&self.directory)
            .expect("should be able to create the data directory if it does not already exist");
        let dir_contents =
            fs::read_dir(&self.directory).expect("Should be able to read the local data directory");

        let mnemonic_files: Vec<_> = dir_contents
            .map(|file| {
                file.expect("should be a valid file")
                    .path()
                    .file_stem()
                    .expect("should have valid stem")
                    .to_str()
                    .expect("should not contain invalid chars")
                    .to_string()
            })
            .collect();
        Self {
            filesystem: FileSystem { mnemonic_files },
            ..self
        }
    }

    pub fn with_new_mnemonic_file(self, filename: String) -> Self {
        let mnemonic_files: Vec<String> = self
            .filesystem
            .mnemonic_files
            .clone()
            .into_iter()
            .chain(vec![filename])
            .collect();
        Self {
            filesystem: FileSystem { mnemonic_files },
            ..self
        }
    }
}

impl Default for State {
    fn default() -> Self {
        use directories::ProjectDirs;
        use std::env;
        let directory = ProjectDirs::from("", "", "mn")
            .expect("Should be able to determine project directory")
            .data_local_dir()
            .to_str()
            .expect("Should be able to find local data directory inside project directory")
            .to_string();
        let editor = if let Some(editor) = env::var_os("VISUAL") {
            editor.into_string().expect("can parse $VISUAL")
        } else if let Some(editor) = env::var_os("EDITOR") {
            editor.into_string().expect("Can parse $EDITOR")
        } else {
            "nano".to_string()
        };
        Self {
            list: List::default(),
            add: AddBuilder::new().editor(editor.clone()).build().unwrap(),
            edit: EditBuilder::new().editor(editor).build().unwrap(),
            show: Show::default(),
            rm: Rm::default(),
            directory,
            mnemonics: Vec::new(),
            filesystem: FileSystem::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone, Default)]
#[builder(setter(into), default)]
pub struct Add {
    blank: bool,
    editor: String,
}

impl Add {
    pub fn blank(&self) -> &bool {
        &self.blank
    }

    pub fn editor(&self) -> &String {
        &self.editor
    }
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone, Default)]
#[builder(setter(into), default)]
pub struct Edit {
    push: Option<String>,
    editor: String,
}

impl Edit {
    pub fn push(&self) -> &Option<String> {
        &self.push
    }
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone, Default)]
#[builder(setter(into), default)]
pub struct List {}

#[derive(Deserialize, Serialize, Builder, Debug, Clone, Default)]
#[builder(setter(into), default)]
pub struct Rm {
    force: bool,
}

impl Rm {
    pub fn force(&self) -> &bool {
        &self.force
    }
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone)]
#[builder(setter(into), default)]
pub struct Show {
    plaintext: bool,
    syntax: String,
    theme: String,
}

impl Show {
    pub fn syntax(&self) -> &String {
        &self.syntax
    }
    pub fn theme(&self) -> &String {
        &self.theme
    }
    pub fn plaintext(&self) -> &bool {
        &self.plaintext
    }
}

impl Default for Show {
    fn default() -> Self {
        Self {
            plaintext: false,
            syntax: "md".to_string(),
            theme: "TwoDark".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone, Default)]
#[builder(setter(into), default)]
pub struct FileSystem {
    mnemonic_files: Vec<String>,
}

impl FileSystem {
    pub fn mnemonic_files(&self) -> &Vec<String> {
        &self.mnemonic_files
    }
}
