
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'mn' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'mn'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'mn' {
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Sets a color scheme for the displayed mnemonic')
            [CompletionResult]::new('--theme', 'theme', [CompletionResultType]::ParameterName, 'Sets a color scheme for the displayed mnemonic')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'The language syntax used for highlighting the output. [Default: md]')
            [CompletionResult]::new('--syntax', 'syntax', [CompletionResultType]::ParameterName, 'The language syntax used for highlighting the output. [Default: md]')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Print the mnemonic with no syntax highlighting at all.')
            [CompletionResult]::new('--plaintext', 'plaintext', [CompletionResultType]::ParameterName, 'Print the mnemonic with no syntax highlighting at all.')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Adds a new, blank mnemonic without opening it for editing')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edits the provided mnemonic')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'Lists all existing mnemonics')
            [CompletionResult]::new('rm', 'rm', [CompletionResultType]::ParameterValue, 'Deletes a mnemonic')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'show the provided mnemonic [DEFAULT]')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'mn;add' {
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Create a new mnemonic by opening it with the editor at PATH')
            [CompletionResult]::new('--editor', 'editor', [CompletionResultType]::ParameterName, 'Create a new mnemonic by opening it with the editor at PATH')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Create a blank mnemonic without opening it in your editor')
            [CompletionResult]::new('--blank', 'blank', [CompletionResultType]::ParameterName, 'Create a blank mnemonic without opening it in your editor')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mn;edit' {
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Pushes a new line to the provided mnemonic')
            [CompletionResult]::new('--push', 'push', [CompletionResultType]::ParameterName, 'Pushes a new line to the provided mnemonic')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Edit the mnemonic with the editor at PATH')
            [CompletionResult]::new('--editor', 'editor', [CompletionResultType]::ParameterName, 'Edit the mnemonic with the editor at PATH')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mn;list' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mn;rm' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'deletes the mnemonic without prompting for confirmation')
            [CompletionResult]::new('--force', 'force', [CompletionResultType]::ParameterName, 'deletes the mnemonic without prompting for confirmation')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mn;show' {
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Sets a color scheme for the displayed mnemonic')
            [CompletionResult]::new('--theme', 'theme', [CompletionResultType]::ParameterName, 'Sets a color scheme for the displayed mnemonic')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'The language syntax used for highlighting the output. [Default: md]')
            [CompletionResult]::new('--syntax', 'syntax', [CompletionResultType]::ParameterName, 'The language syntax used for highlighting the output. [Default: md]')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Print the mnemonic with no syntax highlighting at all.')
            [CompletionResult]::new('--plaintext', 'plaintext', [CompletionResultType]::ParameterName, 'Print the mnemonic with no syntax highlighting at all.')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mn;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
