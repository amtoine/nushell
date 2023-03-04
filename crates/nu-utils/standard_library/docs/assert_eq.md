make sure two operands of the same type have the same value

```nushell
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

>_ assert_eq $a 3
>_ assert_eq $a 1
Error:
  × left is not equal to right
   ╭─[entry #12:5:1]
 5 │     if not $cond {
 6 │         error make {msg: $msg}
   ·         ─────┬────
   ·              ╰── originates from here
 7 │     }
   ╰────
```

Usage:
  > std assert eq <left> <right> 

Flags:
  -h, --help - Display the help message for this command

Parameters:
  left <any>: 
  right <any>: 

