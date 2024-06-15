# Functions

- Declared using `fn` keyword, with statically-typed parameter grouped in paranthesis, followed by curly-brases for function body.

    ```rust
    fn hello(a:i32, b:char) {
        println!("{a} {b}");
    }
    ```
- Statement does not return any values, always ended with semicolon; Expression return a value, does not ended with semicolon.
    - Statement: `let x = 1 + 3;`
    - Expression: `1+3`

- Function return unnamed value, type-denoted with `->`.
    ```rust
    fn five() -> i32 {
        5 // NOTE: expression dont have semicolon
    }

    fn main() {
        let x = five();
        println!("[x}");
    }
    ```
- `return` keyword can be used for early return. Else final expression will be returned.
- function will return unit (`()`) if no expression is found at the end of function
