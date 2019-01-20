use clap::{crate_authors, crate_version, App, Arg, SubCommand};
pub fn build_cli() -> App<'static, 'static> {
    let CliText {
        app,
        edit,
        list,
        new,
        push,
        rm,
        theme,
        theme_options,
    } = CliText::new();
    App::new(app.name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(app.description)
        .arg(
            Arg::with_name("MNEMONIC")
                .help("the mnemonic to display")
                .index(1)
                .conflicts_with("list")
                .required(true),
        )
        .arg(
            Arg::with_name(new.name)
                .help(new.help)
                .long(new.long)
                .short(new.short),
        )
        .arg(
            Arg::with_name(push.name)
                .help(push.help)
                .long(push.long)
                .short(push.short)
                .takes_value(true)
                .number_of_values(1)
                .value_name(push.value_name),
        )
        .arg(
            Arg::with_name(list.name)
                .help(list.help)
                .long(list.long)
                .short(list.short),
        )
        .arg(
            Arg::with_name(rm.name)
                .help(rm.help)
                .long(rm.long)
                .short(rm.short),
        )
        .arg(
            Arg::with_name(theme.name)
                .help(theme.help)
                .long(theme.long)
                .short(theme.short)
                .takes_value(true)
                .value_name(theme.value_name)
                .possible_values(&theme_options),
        )
        .arg(
            Arg::with_name(edit.name)
                .help(edit.help)
                .long(edit.long)
                .short(edit.short),
        )
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
}
pub struct HeaderInfo {
    pub name: &'static str,
    pub description: &'static str,
}
pub struct CliText {
    pub app: HeaderInfo,
    pub list: ArgValues,
    pub new: ArgValues,
    pub edit: ArgValues,
    pub rm: ArgValues,
    pub push: OptValues,
    pub theme: OptValues,
    pub theme_options: Vec<&'static str>,
}

impl CliText {
    pub fn new() -> CliText {
        CliText {
            app: HeaderInfo {
                name: "mnemonic",
                description: "Remembering those little things that slip your mind",
            },
            list: ArgValues {
                name: "list",
                long: "--list",
                short: "-l",
                help: "lists all existing mnemonics",
            },
            new: ArgValues {
                name: "new",
                long: "--new",
                short: "-n",
                help: "adds a new mnemonic",
            },
            edit: ArgValues {
                name: "edit",
                long: "--edit",
                short: "-e",
                help: "edits the provided mnemonic",
            },
            rm: ArgValues {
                name: "rm",
                long: "--rm",
                short: "-r",
                help: "deletes a mnemonic",
            },
            push: OptValues {
                name: "push",
                long: "--push",
                short: "-p",
                help: "pushes a new line to the provided mnemonic",
                value_name: "NEW_TEXT",
                default_value: "",
            },
            theme: OptValues {
                name: "theme",
                long: "--theme",
                short: "-t",
                help: "sets a color scheme for the displayed mnemonic",
                value_name: "COLOR_SCHEME",
                default_value: "",
            },
            theme_options: vec![
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
            ],
        }
    }
}
