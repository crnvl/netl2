# NETL2 Language Convention
## Variables
All variables in NETL2 should be in lower snake case. You cannot use reserved keywords as variable names.
Additionally, variable identifiers cannot contain spaces, special characters, or numbers.

Examples:
```rs
    v var_name = 2 <- Recommended
    v varName = 2 <- Not recommended
    v varname = 2 <- Not recommended
    v var1 = 2 <- Does not work
```

## Functions
All functions in NETL2 should be in lower snake case. You cannot use reserved keywords as function names.
Additionally, function identifiers cannot contain spaces, special characters, or numbers.

Examples:
```rs
    f main {} <- Recommended
    f main_function {}  <- Recommended
    f Main {}  <- Not recommended
    f mainFunction {}  <- Not recommended
    f func1 {}  <- Does not work
```

## Comments
Comments cannot be written at this time.