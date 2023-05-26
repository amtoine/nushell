use error.nu error-fmt

export use operators.nu
export use modules.nu
export use aliases.nu
export use externs.nu
export use commands.nu

# Display help information about different parts of Nushell.
#
# `help word` searches for "word" in commands, aliases and modules, in that order.
#
# Examples:
#   show help for single command, alias, or module
#   > help match
#
#   search for string in command names, usage and search terms
#   > help --find char
export def main [
    ...item: string  # the name of the help item to get help on
    --find (-f): string  # string to find in help items names and usage
] {
    if ($item | is-empty) and ($find | is-empty) {
        print $"Welcome to Nushell.

Here are some tips to help you get started.
  * ('help -h' | nu-highlight) or ('help help' | nu-highlight) - show available ('help' | nu-highlight) subcommands and examples
  * ('help commands' | nu-highlight) - list all available commands
  * ('help <name>' | nu-highlight) - display help about a particular command, alias, or module
  * ('help --find <text to search>' | nu-highlight) - search through all help commands table

Nushell works on the idea of a "(ansi default_italic)pipeline(ansi reset)". Pipelines are commands connected with the '|' character.
Each stage in the pipeline works together to load, parse, and display information to you.

(ansi green)Examples(ansi reset):
    List the files in the current directory, sorted by size
    > ('ls | sort-by size' | nu-highlight)

    Get information about the current system
    > ('sys | get host' | nu-highlight)

    Get the processes on your system actively using CPU
    > ('ps | where cpu > 0' | nu-highlight)

You can also learn more at (ansi default_italic)(ansi light_cyan_underline)https://www.nushell.sh/book/(ansi reset)"
        return
    }

    let target_item = ($item | str join " ")

    let commands = (try { commands $target_item --find $find })
    if not ($commands | is-empty) { return $commands }

    let aliases = (try { aliases $target_item --find $find })
    if not ($aliases | is-empty) { return $aliases }

    let modules = (try { modules $target_item --find $find })
    if not ($modules | is-empty) { return $modules }

    let span = (metadata $item | get span)
    error make {
        msg: ("std::help::item_not_found"  | error-fmt)
        label: {
            text: "item not found"
            start: $span.start
            end: $span.end
        }
    }
}
