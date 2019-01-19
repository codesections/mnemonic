use clap::{crate_authors, crate_version, App, Arg, ArgGroup, SubCommand};
use directories::ProjectDirs;
use prettyprint::*;
use std::process;

fn main() {
    let CliText {
        app,
        edit,
        list,
        new,
        push,
        rm,
        theme,
    } = CliText::new();
    let matches = App::new(app.name)
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
                .short(push.short),
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
                .value_name(theme.value_name),
        )
        .arg(
            Arg::with_name(edit.name)
                .help(edit.help)
                .long(edit.long)
                .short(edit.short),
        )
        .get_matches();

    if let Some(in_file) = matches.value_of("input") {
        let printer = PrettyPrinter::default()
            .header(false)
            .grid(false)
            .language("md")
            .theme("TwoDark")
            .line_numbers(false)
            .build()
            .unwrap();

        if let Some(proj_dir) = ProjectDirs::from("", "", "mn") {
            let data_dir = proj_dir.data_local_dir().to_str().unwrap();
            let file = format!("{}/{}", data_dir, in_file);

            printer.file(file).unwrap_or_else(|_| {
                eprintln!(
                    "{} not found.  Would you like to add it to Mnemonic?",
                    in_file
                );
                process::exit(1);
            });
        }
    }
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
    pub push: ArgValues,
    pub theme: OptValues,
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
                help: "lists all existing mnemonics [UNIMPLEMENTED]",
            },
            new: ArgValues {
                name: "new",
                long: "--new",
                short: "-n",
                help: "adds a new mnemonic [UNIMPLEMENTED]",
            },
            edit: ArgValues {
                name: "edit",
                long: "--edit",
                short: "-e",
                help: "edits the provided mnemonic [UNIMPLEMENTED]",
            },
            rm: ArgValues {
                name: "rm",
                long: "--rm",
                short: "-r",
                help: "deletes a mnemonic [UNIMPLEMENTED]",
            },
            push: ArgValues {
                name: "push",
                long: "--push",
                short: "-p",
                help: "pushes a new line to the provided mnemonic [UNIMPLEMENTED]",
            },
            theme: OptValues {
                name: "theme",
                long: "--theme",
                short: "-t",
                help: "sets a color scheme for the displayed mnemonic [UNIMPLEMENTED]",
                value_name: "COLOR_SCHEME",
                default_value: "",
            },
        }
    }
}
