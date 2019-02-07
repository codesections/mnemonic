
edit:completion:arg-completer[mn] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'mn'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'mn'= {
            cand -t 'Sets a color scheme for the displayed mnemonic'
            cand --theme 'Sets a color scheme for the displayed mnemonic'
            cand -s 'The language syntax used for highlighting the output. [Default: md]'
            cand --syntax 'The language syntax used for highlighting the output. [Default: md]'
            cand -p 'Print the mnemonic with no syntax highlighting at all.'
            cand --plaintext 'Print the mnemonic with no syntax highlighting at all.'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand add 'Adds a new, blank mnemonic without opening it for editing'
            cand edit 'Edits the provided mnemonic'
            cand list 'Lists all existing mnemonics'
            cand rm 'Deletes a mnemonic'
            cand show 'show the provided mnemonic [DEFAULT]'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'mn;add'= {
            cand -e 'Create a new mnemonic by opening it with the editor at PATH'
            cand --editor 'Create a new mnemonic by opening it with the editor at PATH'
            cand -b 'Create a blank mnemonic without opening it in your editor'
            cand --blank 'Create a blank mnemonic without opening it in your editor'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mn;edit'= {
            cand -p 'Pushes a new line to the provided mnemonic'
            cand --push 'Pushes a new line to the provided mnemonic'
            cand -e 'Edit the mnemonic with the editor at PATH'
            cand --editor 'Edit the mnemonic with the editor at PATH'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mn;list'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mn;rm'= {
            cand -f 'deletes the mnemonic without prompting for confirmation'
            cand --force 'deletes the mnemonic without prompting for confirmation'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mn;show'= {
            cand -t 'Sets a color scheme for the displayed mnemonic'
            cand --theme 'Sets a color scheme for the displayed mnemonic'
            cand -s 'The language syntax used for highlighting the output. [Default: md]'
            cand --syntax 'The language syntax used for highlighting the output. [Default: md]'
            cand -p 'Print the mnemonic with no syntax highlighting at all.'
            cand --plaintext 'Print the mnemonic with no syntax highlighting at all.'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mn;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
