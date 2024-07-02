# Error Handling

## Unexpected Error with `panic!`

- `panic!` marco is a way to express unexpected error in a program. By default, rust will print failure message, unwind, clean up stack and quit.
- Add `RUST_BACKTRACE=1` in front of `cargo run` to print out the call stack for debugging purposes.

## Expected Error with `Result<T,E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Result` enum is a way to handle expected error returned from a function. This allows better error handling for cases where program might fail but we don't want the program to crash immediately.
- `Err` enum type can then be handled using `match` expression or other control flow structure and methods for better readability.

    ```rust
    use std::fs::File;
    use std::io::ErrorKind;
    
    fn main() {
        let greeting_file_result = File::open("hello.txt");
    
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }
    use std::fs::File;
    use std::io::ErrorKind;
    
    fn main() {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }
    ```
- Uses `.unwrap()` or `.expect()` to trigger `panic!` immediately

### `?` Operator for Error Propagation

- For a result enum, `?` operator unwrap the `Ok` value and automatically propagate error to the previous function if `Err` is present.
    ```rust
    use std::fs::File;
    use std::io::{self, Read};
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    ```

