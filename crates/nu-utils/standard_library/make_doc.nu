def "path add extension" [extension: string] {
  path parse
  | upsert extension $extension
  | path join
}

use std.nu

$nu.scope.modules
| where name == std
| get commands.0
| each {|cmd|
    help (["std" $cmd] | str join " ")
    | ansi strip
    | save --force ("crates/nu-utils/standard_library/docs" | path join $cmd | path add extension "md" | str replace --all " " "_")
}
