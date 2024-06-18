# Ownership

## What is Ownership?
- Foundational goal of Rust is to ensure program never have undefined behavior. (due to poor memory management etc..).
- Fundamental Rust's Ownership Concepts for memory safety:
    - Variables, Frames, Stack:
        - Variables live in frame (e.g. `{}`).
        - Stack is a sequence of frames, filled with currently-called-functions.
        - A Frame, along with its variables, and owned heap are freed/dropped/deallocated after frame exits.
        - Primitive variable are copied in stack frame when an expression reads a variable
    - Pointer, Heap
        - A variable in stack (or more specifically frame) can hold a pointer to a sequence of data in Heap.
        - Heap is a seperate region of memory where data can live indefinitely, so long as its owner variable exists (i.e. not exited out of stack).
- Manual memory management is troublematic => Rust automatically frees a box's heap memory.
    > **Box deallocation priciple:** If a variable OWNS a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.
        
    ```rust
    let a = Box::new([0;100_000_000]);  // a owns a pointer to Box in heap
    let b = a;  // b is now the new owner of pointer to Box in heap
                // but a still holds the pointer, despite not being the owner
    ```
    - Boxes are used by Rust data structures like `Vec`, `String` and `HashMap` to hold a variable number of elements.
- Variable cannot be used after being moved, to prevent undefined behavior of pointer to unknown heap.
    > **Moved heap data principle:** if a variable `x` moves ownership of heap data to another variable `y`, then `x` cannot be used after the move.

    ```rust
    fn main() {
        let first = String::from("Ferris");
        let full = add_suffix(first);
        println!("{full}, originally {first}"); // ERROR: first points to deallocated memory
    }
    
    fn add_suffix(mut name: String) -> String {
        // Ownership of first is transfer to name
        name.push_str(" Jr.");  // Changes is made to name's heap memory
        name
    }
    ```
- Cloning a variable using `.clone()` creates deep-copy in heap.
    ```rust
    fn main() {
        let first = String::from("Ferris");
        let first_clone = first.clone();
        let full = add_suffix(first_clone);
        println!("{full}, originally {first}");
    }
    
    fn add_suffix(mut name: String) -> String {
        name.pust_str(" Jr.");
        name
    }
    ```
## Ownership in a nutshell:
1. All heap data must be owned by exactly one variable.
2. Rust deallocates heap data once its owner goes out of scope.
3. Ownership can be transferred by moves, which happens on assignments and function calls.
4. Heap data can only be accessed through its current owner, not a previous owner.

