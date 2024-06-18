# Fixing Ownership Errors

This chapter will go through a few-case studies of problematic ownership errors and how to fix them.

### Returning a Reference to the Stack
- Code:
    ```rust
    fn return_a_string() -> &String {
        let s = String::from("Hello world");
        &s
    }
- Error:
    - `s` is deallocated when function is returned: violate "Data Must Outlive All Of Its Reference"
- Solution:
    1. Return ownership of string out of the function, changing `&String` to `String`
        ```rust
        fn return_a_string() -> String {
            let s = String::from("Hello world");
            s
        }
        ```
    2. Return string literal, which lives forever (indicated by 'static)
        ```rust
        fn return_a_string() -> &'static str {
            "Hello world"
        }
        ```
    3. Defer borrow-checking to runtime using garbage collection
        ```rust
        use std::rc::Rc;
        fn return_a_string() -> Rc<String> {
            let s = Rc::new(String::from("Hello world"));
            Rc::clone(&s)
        }
        ```
### Not Enough Permissions
- Code:
    ```rust
    fn stringify_name_with_title(name: &Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join(" ");
        full
    }
    ```
- Error:
    - `name` does not have write/mutate permission.
    - Changing reference to mutable reference, `&mut Vec<String>` could lead to deallocation of original memory.
    - Passing ownership of heap-owning data structures is rare as it makes parent variable unusable.

- Solution:
    1. Use `.clone()` to create a mutable and owned variable from `name` (create unnecessary copies)
       ```rust
        fn stringify_name_with_title(name: &Vec<String>) -> String {
            let mut name_clone = name.clone();
            name_clone.push(String::from("Esq."));
            let full = name_clone.join(" ");
            full
        } 
        ```
    2. Use `slice::join` to copies data into `full` directly
        ```rust
        fn stringify_name_with_title(name: &Vec<String>) -> String {
            let mut full = name.join(" ");
            full.push_str(" Esq.");
            full
        }
        ```
### Aliasing and Mutation a Data Structure

- Code:
    ```rust
    fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
        let largest: &String = 
          dst.iter().max_by_key(|s| s.len()).unwrap();
        for s in src {
            if s.len() > largest.len() {
                dst.push(s.clone());
            }
        }
    }
    ```
- Porblem:
    - `dst.push(..)` could deallocate the contents of `dst`, invalidating the reference `largest`.

- Solutions:
    1. Clone `largest` (Performance hit for allocating and copying string data)
        ```rust
        fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
            let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
            for s in src {
                if s.len() > largest.len() {
                    dst.push(s.clone());
                }
            }
        }
        ```
    2. Perform all the length comparisons first, and then mutate dst afterwards (performance hit for allocating vector `to_add`)
        ```rust
        fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
            let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
            let to_add: Vec<String> = 
                src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
            dst.extend(to_add);
        }
        ```
    3. Copy out the length of largest, since we don't actually need the contents of largest, just its length
        ```rust
        fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
            let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
            for s in src {
                if s.len() > largest_len {
                    dst.push(s.clone());
                }
            }
        }
        ```
    - Key Idea: Shortening lifetime of borrows on `dst` to not overlap with mutation to `dst`

### Copying vs Moving Out of a Collection
- Code:
```rust
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];
let s: String = *s_ref;

// These drops are normally implicit, but we've added them for clarity.
drop(s);
drop(v);
```

- Problem:
    - double-free problem, after `s` is dropped, "Hello world" is deallocated, and v will have nothing to drop.

- Solution:
    - Avoid taking ownership of string and jut use immutable reference
        ```rust
        let v: Vec<String> = vec![String::from("Hello world")];
        let s_ref: &String = &v[0];
        println!("{s_ref}!");
        ```
    - Clone data you want to get ownership while leaving vector alone
        ```rust
        let v: Vec<String> = vec![String::from("Hello world")];
        let mut s: String = v[0].clone();
        s.push('!');
        println!("{s}");
        ```
    - Use method like `Vec::remove` to move string out of vector
        ```rust
        let mut v: Vec<String> = vec![String::from("Hello world")];
        let mut s: String = v.remove(0);
        s.push('!');
        println!("{s}");
        assert!(v.len() == 0);
        ```
- Summary:
    - If a value does not own heap data, then it can be copied without a move. For example:
        - An i32 does not own heap data, so it can be copied without a move.
        - A String does own heap data, so it can not be copied without a move.
        - An &String does not own heap data, so it can be copied without a move.


### Mutating Different Array Element
- Code:
    ```rust
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    let y = &a[2];
    *x += *y;
    ```
- Problem:
    - Rust borrow checker checks permission for `a[_]`. This causes `x` losing W permission as borrowed by `y`.
- Solution:
    - Use `slice::split_at_mut` to create two array, which escape issue of borrow checker.
    ```rust
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
    ```
    - Use `unsafe` blocks to allow use of "raw" pointers, which are not checked for safety by borrow checker
    ```rust
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
    ```
    > Unsafe code is sometimes necessary to work around the limitations of the borrow checker. As a general strategy, let's say the borrow checker rejects a program you think is actually safe. Then you should look for standard library functions (like `split_at_mut`) that contain unsafe blocks which solve your problem. We will discuss unsafe code further in Chapter 19. For now, just be aware that unsafe code is how Rust implements certain otherwise-impossible patterns.
