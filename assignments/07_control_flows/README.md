# Flow Control

## IFs

- `if condition { }` where `condition` must be a boolean (i.e. `if 1 {};` will throw error).
- `else if condition` and `else condition` can be used for multiple condition matching.
- can be used as expression of same type when assigned to variable. (e.g. `let x = if 1>2 { 1 } else { 2 };`)

## Loops

### `loop`

- Loop a block of code forever until explicitly stopped with `break` or skipped with `continue`.
    ```rust
    fn main() {
        loop {
            println!("forever single");
        }
    }
    ```
- Loop is an expression and can return value after `break` statement.
- Apply `'name` label before loop to modify behavior of `break` and `continue` that is targetted to inner-most loop by default.
    ```rust
    fn main() {
        let mut count = 0;
        'outer_counting_up: loop {
            let mut remaining = 10;
            
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'outer_counting_up;  // Break outer loop with label
                }
                remaining -= 1;
            }
            
            count += 1
        }
    }
    ```


### `while` loop

- `while condition {}`, evaluate condition: boolean on begining of every loop.

### `for` loop

- `for element in array {}` to loop through every element in array.
- `(start..end)` denotes a `Range` that can be used in for-loop.
- `(start..end).rev()` reverse a range.
