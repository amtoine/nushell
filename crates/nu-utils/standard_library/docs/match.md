```nushell
>_ let branches = {
)))     1: { print "this is the 1st branch"}
)))     2: { print "this is the 2nd branch" }
)))     3: { print "this is the 3rd branch" }
)))     4: { print "this is the 4th branch" }
))) }

>_ match 1 $branches
))) match 2 $branches
))) match 3 $branches
))) match 4 $branches
))) match 5 $branches
this is the 1st branch
this is the 2nd branch
this is the 3rd branch
this is the 4th branch

>_ match 1 $branches { "this is the default branch" }
))) match 2 $branches { "this is the default branch" }
))) match 3 $branches { "this is the default branch" }
))) match 4 $branches { "this is the default branch" }
))) match 5 $branches { "this is the default branch" }
this is the 1st branch
this is the 2nd branch
this is the 3rd branch
this is the 4th branch
this is the default branch
```

Usage:
  > std match <input> <matchers> (default) 

Flags:
  -h, --help - Display the help message for this command

Parameters:
  input <string>: 
  matchers <record>: 
  (optional) default <block>: 

