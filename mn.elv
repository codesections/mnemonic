
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
            cand -p 'Pushes a new line to the provided mnemonic'
            cand --push 'Pushes a new line to the provided mnemonic'
            cand -t 'Sets a color scheme for the displayed mnemonic'
            cand --theme 'Sets a color scheme for the displayed mnemonic'
            cand -a 'Adds a new, blank mnemonic without opening it for editing'
            cand --add 'Adds a new, blank mnemonic without opening it for editing'
            cand -n 'Adds a new mnemonic and opens it in your editor'
            cand --new 'Adds a new mnemonic and opens it in your editor'
            cand -l 'Lists all existing mnemonics'
            cand --list 'Lists all existing mnemonics'
            cand -r 'Deletes a mnemonic'
            cand --rm 'Deletes a mnemonic'
            cand -e 'Edits the provided mnemonic'
            cand --edit 'Edits the provided mnemonic'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
