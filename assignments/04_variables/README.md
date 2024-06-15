# Variables and Mutability

- `let` declare immutable variable.
- `let mut` mutable variable.
- `const` is immutable with compulsary type declaration.
- `const` must be named with `UPPERCASE_DELIMITED_BY_UNDERSCORE`.
- `const` can be declared as expression and some expression will be evaluated during compile time. 
- Shadowing creates transformation of a variable, regardless of its immutability and datatype.
- Shadowing is scope dependent. (i.e. shadow in inner-scope does not influence shadow in outer-scope; shadow in outer-scope influence shadow in inner-scope)
- Shadowing is useful trick to avoid writing extra variable of same name. (?still need see actual example of its usefulness thru project)
