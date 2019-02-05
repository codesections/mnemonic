pub const TOML: &str = r#"#                                       __ _       
#                                       __ _       
#  _ __ ___  _ __       ___ ___  _ __  / _(_) __ _ 
# | '_ ` _ \| '_ \     / __/ _ \| '_ \| |_| |/ _` |
# | | | | | | | | |   | (_| (_) | | | |  _| | (_| |
# |_| |_| |_|_| |_|    \___\___/|_| |_|_| |_|\__, |
#                                            |___/ 
#  This file allows you to configure mn.  Specifically, you can configure any
#  option that you could otherwise set with a command-line flag.  For example,
#  if you set `force = true` in the `[rm]` section below, then running `mn rm`
#  will never prompt you for confirmation—it will always act as though you had
#  passed the `--force` flag to the `rm` command.


# An array of default mnemonics to use with every st commands act only on the
# first mnemonic in the array.
#     ## Example:
#     mnemonics = ["notes"] 
#     # Running `mn` will now display the notes mnemonic
mnemonics = []

# The directory in which to store your mnemonics.
# NOTE: if you change this value after creating mnemonics, you will need to
# copy your mnemonics from the old directory.
#     ## Example:
#     directory = "~/..mnemonics"
directory = "SET_DYNAMICALLY_BASED_ON_ENV_VARS"


# The following variables apply to the `mn add` command
[add]
# Create a blank mnemonic without opening it for # editing.
blank = false

# The editor used to create new mnemonics
# NOTE: this may be different than the editor used to edit existing mnemonics
editor = "SET_DYNAMICALLY_BASED_ON_ENV_VARS" 


# The following variables apply to the `mn edit` command
[edit]
# The editor used to edit existing mnemonics
# NOTE: this may be different than the editor used to create new mnemonics
editor = "SET_DYNAMICALLY_BASED_ON_ENV_VARS"


# The following variables apply to the `mn list` command
[list]
# (None right now)


# The following variables apply to the `mn rm` command
[rm]
# Remove a mnemonic without prompting for confirmation
force = false


# The following variables apply to the `mn rm` command
[show]
# Print all mnemonics as plaintext (e.g., without syntax highlighting)
plaintext = false

# The default syntax highlighting
syntax = "md"

# The default color-scheme
theme = "TwoDark"


[filesystem]

mnemonic_files = []"#;
