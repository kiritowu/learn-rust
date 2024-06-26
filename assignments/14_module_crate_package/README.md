# Module Crate and Packages

- Module:
    - Made up of code in a file
    - Code inside module are private by default. Use `pub` keyword to make a `fn`, `struct` or `mod` public.
- Crate:
    - Multiple module forms a crate. Smallest amount of code that Rust compiler considers at a time.
    - There are two types of creates: binary crate (i.e. executable with `fn main` like `src/main.rs`) and library crate (e.g. `src/lib.rs`).
    - In the crate root file, related modules are defined using `mod` keyword.
    - Use `use` keyword to creates shortcuts to items to reduce repetition of long paths.
- Packages:
    - Bundle of one or more crates that provides a set of functionality.
    - Contains `Cargo.toml` to describe how to build those crates.

### Paths for Referring to an Item in Module Tree

- To call a function, we need to know its path using one of the two methods, seperated by `::`:
    - Absolute path: Full path that starts with literal `crate`
    - Relative path: Starts from the current module and uses `self`, `super`, or an identifier in the current module.
    ```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
    
        // Relative path
        front_of_house::hosting::add_to_waitlist();
    
        // Simplify scope with use (More idiomatic)
        hosting::add_to_waitlist();
       ```

- To use an external package available in [crates.io](creates.io), one needs to:
    1. Add `package_name` as dependencies in `Cargo.toml`
    2. Import them with `use` statement.

### Use Nested Path to Clean Up Large use Lists

```rust
use std::io;
use std::io::Write;

use std::io::{self, Write};

use std::collections::*;
```

