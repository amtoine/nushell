def "get commands" [] {
    help commands
    | where command_type == builtin
    | get name
}

def "get help" [cmd: string] {
    cargo run -- -c $"help ($cmd)"
    | ansi strip
    | lines
}

def "extract examples" [--command: string] {
    split list 'Examples:'
    | try {
        get 1
        | str trim
        | reverse
        | skip 1
        | reverse
        | split list ""
        | enumerate
        | each {|it|
            {
                command: $command
                id: $it.index
                example: ($it.item | skip 1 | str replace "> " "" | find --invert --regex '^>>')
                output: ($it.item | find --regex '^>>' | str replace --all '^>>' '')
            }
        }
    }
}

export def "analyze examples" [] {
    cargo build --release

    get commands
    | each {|cmd|
        get help $cmd | extract examples --command $cmd
    }
    | flatten
    | str trim
    | to json
}
