```nushell
>_ assert ($a == 3)
>_ assert ($a != 3)
Error:
  × condition given to `assert` does not hold
   ╭─[entry #12:5:1]
 5 │     if not $cond {
 6 │         error make {msg: $msg}
   ·         ─────┬────
   ·              ╰── originates from here
 7 │     }
   ╰────
```

Usage:
  > std assert <cond> 

Subcommands:
  std assert eq - ```nushell
>_ assert_eq $a "a string"
Error:
  × left and right operand of `assert eq` should have the same type
   ╭─[entry #12:5:1]
 5 │     if not $cond {
 6 │         error make {msg: $msg}
   ·         ─────┬────
   ·              ╰── originates from here
 7 │     }
   ╰────
  std assert ne - ```nushell
>_ assert_ne $a "a string"
Error:
  × left and right operand of `assert eq` should have the same type
   ╭─[entry #12:5:1]
 5 │     if not $cond {
 6 │         error make {msg: $msg}
   ·         ─────┬────
   ·              ╰── originates from here
 7 │     }
   ╰────

Flags:
  -h, --help - Display the help message for this command

Parameters:
  cond <bool>: 

