use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};
pub fn build_cli() -> App<'static, 'static> {
    let CliText {
        app,
        add,
        edit,
        list,
        push,
        rm,
        theme,
        ..
    } = CliText::new();
    App::new(app.name)
        .setting(AppSettings::SubcommandsNegateReqs)
        .version(crate_version!())
        .author(crate_authors!())
        .about(app.description)
        .subcommand(
            SubCommand::with_name(add.name)
                .about(add.help)
                .arg(
                    Arg::with_name("blank")
                        .help("Create a blank mnemonic without opening it in your editor")
                        .long("--blank")
                        .short("-b"),
                )
                .arg(
                    Arg::with_name("editor")
                        .help("Create a new mnemonic by opening it with the editor at PATH")
                        .takes_value(true)
                        .value_name("PATH")
                        .long("--editor")
                        .short("-e"),
                )
                .arg(
                    Arg::with_name("MNEMONIC")
                        .help("The name of the mnemonic to add")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(edit.name)
                .about(edit.help)
                .arg(
                    Arg::with_name("push")
                        .help(push.help)
                        .long(push.long)
                        .short(push.short)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("editor")
                        .help("Edit the mnemonic with the editor at PATH")
                        .takes_value(true)
                        .value_name("PATH")
                        .long("--editor")
                        .short("-e"),
                )
                .arg(
                    Arg::with_name("MNEMONIC")
                        .help("The name of the mnemonic to edit")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name(list.name).about(list.help))
        .subcommand(
            SubCommand::with_name(rm.name)
                .about(rm.help)
                .arg(
                    Arg::with_name("force")
                        .help("deletes the mnemonic without prompting for confirmation")
                        .long("--force")
                        .short("-f"),
                )
                .arg(
                    Arg::with_name("MNEMONIC")
                        .help("The mnemonic or mnemonics to delete")
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("show the provided mnemonic [DEFAULT]")
                .arg(
                    Arg::with_name(theme.name)
                        .help(theme.help)
                        .long(theme.long)
                        .short(theme.short)
                        .takes_value(true)
                        .possible_values(&theme.possible_values),
                )
                .arg(
                    Arg::with_name("plaintext")
                        .help("Print the mnemonic with no syntax highlighting at all.")
                        .long("--plaintext")
                        .short("-p")
                        .conflicts_with("syntax"),
                )
                .arg(
                    Arg::with_name("syntax")
                        .help("The language syntax used for highlighting the output. [Default: md]")
                        .long("--syntax")
                        .short("-s")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("MNEMONIC")
                        .help("The name of the mnemonic to print to the console")
                        .required(true),
                ),
        )
        .arg(
            Arg::with_name("MNEMONIC")
                .help("the mnemonic to display")
                .required(true),
        )
        .arg(
            Arg::with_name(theme.name)
                .help(theme.help)
                .long(theme.long)
                .short(theme.short)
                .takes_value(true)
                .possible_values(&theme.possible_values)
                .hidden(true),
        )
        .arg(
            Arg::with_name("plaintext")
                .help("Print the mnemonic with no syntax highlighting at all.")
                .long("--plaintext")
                .short("-p")
                .conflicts_with("syntax")
                .hidden(true),
        )
        .arg(
            Arg::with_name("syntax")
                .help("The language syntax used for highlighting the output. [Default: md]")
                .long("--syntax")
                .short("-s")
                .takes_value(true)
                .hidden(true),
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
    pub possible_values: Vec<&'static str>,
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
                possible_values: Vec::new(),
            },
            theme: OptValues {
                name: "theme",
                long: "--theme",
                short: "-t",
                help: "Sets a color scheme for the displayed mnemonic",
                value_name: "COLOR_SCHEME",
                default_value: "",
                possible_values: vec![
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
            },
        }
    }
}
