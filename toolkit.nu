def get_commands_with_examples [--verbose (-v): bool = false] {
    let commands = (
        $nu.scope.commands
        | where is_builtin
        | select name examples
    )

    if $verbose {
        print "removing following commands that do not have examples:"
        $commands | where {|x| ($x.examples | length) == 0} | get name
    }

    let commands_with_examples = (
        $commands
        | where {|x| ($x.examples | length) > 0}
    )

    $commands_with_examples
}

def get-true-output-data-length [] {
    nu -c $"($in | to nuon) | table -e"
    | lines
    | length
}

export def "analyze examples" [--verbose (-v): bool] {
    let commands = (get_commands_with_examples --verbose $verbose)

    let start = (date now)

    let report = (
        $commands
        | upsert examples {|x|
            if $verbose { print -n $"(ansi erase_line)($x.name)\r" }
            $x.examples
            | merge (
                $x.examples.result
                | each { get-true-output-data-length }
                | wrap len
            )
        }
    )

    if $verbose { print $"done in (ansi yellow_bold)((date now) - $start)(ansi reset)!" }
    $report
}
