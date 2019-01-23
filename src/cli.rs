use clap::{crate_authors, crate_version, App, Arg};
pub fn build_cli() -> App<'static, 'static> {
    let CliText {
        app,
        add,
        edit,
        list,
        new,
        push,
        rm,
        theme,
        ..
    } = CliText::new();
    let mut app = App::new(app.name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(app.description)
        .arg(
            Arg::with_name("MNEMONIC")
                .help("the mnemonic to display")
                .index(1)
                .conflicts_with("list")
                .required(true)
                .index(1),
        );

    for arg in [add, new, list, rm, edit].iter() {
        app = app.arg(
            Arg::with_name(arg.name)
                .help(arg.help)
                .long(arg.long)
                .short(arg.short),
        );
    }
    for opt in [push, theme].iter() {
        let mut arg = Arg::with_name(opt.name)
            .help(opt.help)
            .long(opt.long)
            .short(opt.short)
            .takes_value(true)
            .value_name(opt.value_name);

        if let Some(possible_values) = &opt.possible_values {
            arg = arg.possible_values(&possible_values);
        }
        app = app.arg(arg)
    }
    app
}

pub struct ArgValues {
    pub name: &'static str,
    pub long: &'static str,
    pub short: &'static str,
    pub help: &'static str,
}
pub struct OptValues {
    pub name: &'static str,
    pub long: &'static str,
    pub short: &'static str,
    pub help: &'static str,
    pub value_name: &'static str,
    pub default_value: &'static str,
    pub possible_values: Option<Vec<&'static str>>,
}
pub struct HeaderInfo {
    pub name: &'static str,
    pub description: &'static str,
}
pub struct CliText {
    pub app: HeaderInfo,
    pub add: ArgValues,
    pub list: ArgValues,
    pub help: ArgValues,
    pub new: ArgValues,
    pub edit: ArgValues,
    pub rm: ArgValues,
    pub push: OptValues,
    pub theme: OptValues,
    pub version: ArgValues,
}

impl CliText {
    pub fn new() -> CliText {
        CliText {
            app: HeaderInfo {
                name: "mnemonic",
                description: "Remembering those little things that slip your mind",
            },
            help: ArgValues {
                name: "help",
                long: "--help",
                short: "-h",
                help: "Prints the help information",
            },
            version: ArgValues {
                name: "version",
                short: "-V",
                long: "--version",
                help: "Prints version information.",
            },
            list: ArgValues {
                name: "list",
                long: "--list",
                short: "-l",
                help: "Lists all existing mnemonics",
            },
            add: ArgValues {
                name: "add",
                long: "--add",
                short: "-a",
                help: "Adds a new, blank mnemonic without opening it for editing",
            },
            new: ArgValues {
                name: "new",
                long: "--new",
                short: "-n",
                help: "Adds a new mnemonic and opens it in your editor",
            },
            edit: ArgValues {
                name: "edit",
                long: "--edit",
                short: "-e",
                help: "Edits the provided mnemonic",
            },
            rm: ArgValues {
                name: "rm",
                long: "--rm",
                short: "-r",
                help: "Deletes a mnemonic",
            },
            push: OptValues {
                name: "push",
                long: "--push",
                short: "-p",
                help: "Pushes a new line to the provided mnemonic",
                value_name: "NEW_TEXT",
                default_value: "",
                possible_values: None,
            },
            theme: OptValues {
                name: "theme",
                long: "--theme",
                short: "-t",
                help: "Sets a color scheme for the displayed mnemonic",
                value_name: "COLOR_SCHEME",
                default_value: "",
                possible_values: Some(vec![
                    "1337",
                    "DarkNeon",
                    "GitHub",
                    "Monokai Extended",
                    "Monokai Extended Bright",
                    "Monokai Extended Light",
                    "Monokai Extended Origin",
                    "OneHalfDark",
                    "OneHalfLight",
                    "Sublime Snazzy",
                    "TwoDark",
                    "zenburn",
                ]),
            },
        }
    }
}
