def main [] {
    cargo build --release

    help commands
    | where command_type == builtin
    | get name
    | each {|cmd|
        cargo run -- -c $"help ($cmd)"
        | ansi strip
        | lines
        | split list 'Examples:'
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
                    comamnd: $cmd
                    id: $it.index
                    example: ($it.item | skip 1 | str replace "> " "" | find --invert --regex '^>>')
                    output: ($it.item | find --regex '^>>' | str replace --all '^>>' '')
                }
            }
        }
    }
    | flatten
    | str trim
    | to json
}
