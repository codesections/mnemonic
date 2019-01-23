
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
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Pushes a new line to the provided mnemonic')
            [CompletionResult]::new('--push', 'push', [CompletionResultType]::ParameterName, 'Pushes a new line to the provided mnemonic')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Sets a color scheme for the displayed mnemonic')
            [CompletionResult]::new('--theme', 'theme', [CompletionResultType]::ParameterName, 'Sets a color scheme for the displayed mnemonic')
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'Adds a new, blank mnemonic without opening it for editing')
            [CompletionResult]::new('--add', 'add', [CompletionResultType]::ParameterName, 'Adds a new, blank mnemonic without opening it for editing')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'Adds a new mnemonic and opens it in your editor')
            [CompletionResult]::new('--new', 'new', [CompletionResultType]::ParameterName, 'Adds a new mnemonic and opens it in your editor')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Lists all existing mnemonics')
            [CompletionResult]::new('--list', 'list', [CompletionResultType]::ParameterName, 'Lists all existing mnemonics')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Deletes a mnemonic')
            [CompletionResult]::new('--rm', 'rm', [CompletionResultType]::ParameterName, 'Deletes a mnemonic')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Edits the provided mnemonic')
            [CompletionResult]::new('--edit', 'edit', [CompletionResultType]::ParameterName, 'Edits the provided mnemonic')
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
