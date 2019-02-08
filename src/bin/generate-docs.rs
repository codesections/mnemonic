use clap::Shell;
use man::prelude::*;
use std::fs::File;
use std::io::prelude::*;

include!("../cli.rs");
include!("../err.rs");

fn main() {
    run().unwrap_or_else(|e| e.handle_err())
}

fn run() -> Result<(), CliErr> {
    let mut app = build_cli();
    for shell in [
        Shell::Bash,
        Shell::Elvish,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Zsh,
    ]
    .into_iter()
    {
        app.gen_completions("mn", *shell, "./completions");
    }

    let CliText {
        add,
        list,
        new,
        edit,
        app,
        rm,
        push,
        theme,
        help,
        version,
    } = CliText::new();

    let mut msg = Manual::new(app.name)
        .about(app.description)
        .option(
            Opt::new(push.value_name)
                .short(push.short)
                .long(push.long)
                .help(push.help)
        )
        .option(
            Opt::new(theme.value_name)
                .short(theme.short)
                .long(theme.long)
                .help(
                    &format!(
                        "{} \n.nf\n.B     Possible values: \n    {}\n.fi\nIt is currently not possible to customize the theme beyond these presets, but it is an issue under consideration.",
                        theme.help,
                        theme.possible_values.join("\n    ")
                )
        ))
        .custom(
            Section::new("syntax highlighting")
            .paragraph("By default, mnemonic will highlight all text as Markdown.  If you would like to highlight a portion of a mnemonic based on different rules, you can specify the language with GitHub-style triple backticks.  For example, to highlight Rust code:")
            .paragraph(".nf\n```rust\nlet foo = \"bar\";\n```\n.fi")
        )
        .example(
            Example::new()
            .text("Add a new mnemonic with the name 'notes':")
            .command("mn --add notes")
        )
        .example(
            Example::new()
            .text("Add an item to 'notes' without opening it in your editor:\n.nf")
            .command("mn notes --push \"# Jokes\nThere are two hard problems in computer science: cache invalidation, naming things, and off-by-one errors\"")
        )
        .example(
            Example::new()
            .text("Print the mnemonic 'notes' in your terminal:")
            .command("mn notes")
        )
        .author(Author::new(crate_authors!()));

    for flag in [add, edit, help, list, new, rm, version].iter() {
        msg = msg.flag(
            Flag::new()
                .help(flag.help)
                .long(flag.long)
                .short(flag.short),
        );
    }
    let msg = msg.render();
    let mut file = File::create("./docs/mn.1")
        .map_err(|e| CliErr::CannotGenerateManPage("./docs/mn.1".to_string(), e))?;
    file.write_all(msg.as_bytes())
        .map_err(|e| CliErr::CannotGenerateManPage("./docs/mn.1".to_string(), e))?;
    Ok(())
}
