make sure a boolean condition is true

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
  std assert eq - make sure two operands of the same type have the same value
  std assert ne - make sure two operands of the same type DO NOT have the same value

Flags:
  -h, --help - Display the help message for this command

Parameters:
  cond <bool>: 

