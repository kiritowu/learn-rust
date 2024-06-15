# Functions

- Declared using `fn` keyword, with statically-typed parameter grouped in paranthesis, followed by curly-brases for function body.

    ```rust
    fn hello(a:i32, b:char) {
        println!("{a} {b}");
    }
    ```
- Statement does not return any values, always ended with semicolon; Expression return a value, does not be ended with semicolon.
    - Statement: `let x = 1 + 3;`
    - Expression: `1+3`

- Function can return value that are unnamed, type-denoted with `->`.
- `return` keyword can be used for early return. Else final expression will be returned.
    ```rust
    fn five() -> i32 {
        5 // NOTE: expression dont have semicolon
    }

    fn main() {
        let x = five();
        println!("[x}");
    }
    ```
- function will return unit (`()`) if no expression is found at the end of function
