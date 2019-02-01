use clap::ArgMatches;
use std::{env, ffi, fs};

#[derive(Debug, Default)]
pub struct FsState {
    editor: Option<ffi::OsString>,
    data_dir: Option<String>,
    dir_contents: Option<fs::ReadDir>,
    mn_files: Option<Vec<String>>,
}

impl FsState {
    pub fn from_filesystem() -> Self {
        let editor = if let Some(editor) = env::var_os("VISUAL") {
            Some(editor)
        } else {
            env::var_os("EDITOR")
        };
        use directories::ProjectDirs;
        let data_dir = ProjectDirs::from("", "", "mn")
            .expect("Should be able to determine project directory")
            .data_local_dir()
            .to_str()
            .expect("Should be able to find local data directory inside project directory")
            .to_string();

        fs::create_dir_all(&data_dir)
            .expect("should be able to create the data directory if it does not already exist");
        let dir_contents =
            fs::read_dir(&data_dir).expect("Should be able to read the local data directory");

        let mn_files = dir_contents
            .map(|file| {
                file.expect("should be a valid file")
                    .file_name()
                    .into_string()
                    .expect("should not contain invalid chars")
            })
            .collect();

        Self {
            editor,
            data_dir: Some(data_dir),
            dir_contents: None,
            mn_files: Some(mn_files),
        }
    }
    pub fn editor(&self) -> &Option<ffi::OsString> {
        &self.editor
    }
    pub fn data_dir(&self) -> &Option<String> {
        &self.data_dir
    }
    pub fn dir_contents(self) -> Option<fs::ReadDir> {
        self.dir_contents
    }
    pub fn mn_files(&self) -> &Option<Vec<String>> {
        &self.mn_files
    }
    pub fn add_mn_file(self, mn_file: String) -> Self {
        let mut current_mn_files = self.mn_files.unwrap_or(Vec::new());
        current_mn_files.push(mn_file);

        Self {
            mn_files: Some(current_mn_files),
            ..self
        }
    }

    #[cfg(test)]
    pub fn from_test_data(test_data: TestFsState) -> Self {
        Self {
            editor: test_data.editor,
            data_dir: test_data.data_dir,
            dir_contents: test_data.dir_contents,
            mn_files: test_data.mn_files,
        }
    }
}

#[cfg(test)]
#[derive(Debug, Default)]
pub struct TestFsState {
    editor: Option<ffi::OsString>,
    data_dir: Option<String>,
    dir_contents: Option<fs::ReadDir>,
    mn_files: Option<Vec<String>>,
}

#[cfg(test)]
impl TestFsState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn editor(self, editor: &str) -> Self {
        Self {
            editor: Some(editor.into()),
            ..self
        }
    }

    pub fn data_dir(self, data_dir: &str) -> Self {
        Self {
            data_dir: Some(data_dir.to_string()),
            ..self
        }
    }

    pub fn dir_contents(self, path: &str) -> Self {
        Self {
            dir_contents: Some(std::fs::read_dir(path).expect("Passed by test")),
            ..self
        }
    }

    pub fn mn_files(self, mn_files: Vec<String>) -> Self {
        Self {
            mn_files: Some(mn_files),
            ..self
        }
    }
}

#[derive(Default, Debug)]
pub struct MnArgs {
    mn: Option<String>,
    mnemonics: Option<Vec<String>>,
    push: Option<String>,
    blank_flag: bool,
    force_flag: bool,
    plaintext_flag: bool,
    theme: Option<String>,
    syntax: Option<String>,
}

impl MnArgs {
    pub fn build_from_clap_args(clap_args: &ArgMatches) -> Self {
        Self {
            mn: clap_args.value_of("MNEMONIC").map(|s| s.to_string()),
            theme: clap_args.value_of("theme").map(|s| s.to_string()),
            syntax: clap_args.value_of("syntax").map(|s| s.to_string()),
            mnemonics: match clap_args.values_of("MNEMONIC") {
                Some(clap_vec) => Some(clap_vec.map(|s| s.to_string()).collect()),
                None => None,
            },
            push: clap_args.value_of("push").map(|s| s.to_string()),
            blank_flag: clap_args.is_present("blank"),
            force_flag: clap_args.is_present("force"),
            plaintext_flag: clap_args.is_present("plaintext"),
        }
    }

    #[cfg(test)]
    pub fn from_test_data(test_data: TestMnArgs) -> Self {
        Self {
            mn: test_data.mn,
            theme: test_data.theme,
            syntax: test_data.syntax,
            mnemonics: test_data.mnemonics,
            push: test_data.push,
            blank_flag: test_data.blank_flag,
            force_flag: test_data.force_flag,
            plaintext_flag: test_data.plaintext_flag,
        }
    }

    pub fn mn(&self) -> &Option<String> {
        &self.mn
    }
    pub fn theme(&self) -> &Option<String> {
        &self.theme
    }
    pub fn syntax(&self) -> &Option<String> {
        &self.syntax
    }
    pub fn mnemonics(&self) -> &Option<Vec<String>> {
        &self.mnemonics
    }
    pub fn push(&self) -> &Option<String> {
        &self.push
    }
    pub fn blank_flag(&self) -> &bool {
        &self.blank_flag
    }
    pub fn force_flag(&self) -> &bool {
        &self.force_flag
    }
    pub fn plaintext_flag(&self) -> &bool {
        &self.plaintext_flag
    }
}

#[cfg(test)]
#[derive(Default, Debug)]
pub struct TestMnArgs {
    pub mn: Option<String>,
    pub push: Option<String>,
    pub theme: Option<String>,
    pub syntax: Option<String>,
    pub mnemonics: Option<Vec<String>>,
    pub blank_flag: bool,
    pub force_flag: bool,
    pub plaintext_flag: bool,
}

#[cfg(test)]
impl TestMnArgs {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mn(self, mn: &str) -> Self {
        Self {
            mn: Some(mn.to_string()),
            ..self
        }
    }
    pub fn push(self, push: &str) -> Self {
        Self {
            push: Some(push.to_string()),
            ..self
        }
    }
    pub fn theme(self, theme: &str) -> Self {
        Self {
            theme: Some(theme.to_string()),
            ..self
        }
    }
    pub fn syntax(self, syntax: &str) -> Self {
        Self {
            syntax: Some(syntax.to_string()),
            ..self
        }
    }
    pub fn mnemonics(self, mnemonics: Vec<&str>) -> Self {
        Self {
            mnemonics: Some(mnemonics.iter().map(|s| s.to_string()).collect()),
            ..self
        }
    }
    pub fn blank_flag(self, blank_flag: bool) -> Self {
        Self { blank_flag, ..self }
    }
    pub fn force_flag(self, force_flag: bool) -> Self {
        Self { force_flag, ..self }
    }
    pub fn plaintext_flag(self, plaintext_flag: bool) -> Self {
        Self {
            plaintext_flag,
            ..self
        }
    }
}
