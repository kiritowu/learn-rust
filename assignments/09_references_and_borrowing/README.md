# References and Borrowing

## References and Dereferences
### References Are Non-Owning Pointer
- References (denoted with ampercent `&` operator), points to a variable, without owning a variable.
    ![anya-pointer-meme](https://imgflip.com/i/8u73e1)
- Non-owning property of reference prevent memory being freed from heap after its frame is removed. (See Box deallocation principle for reminder)
    ```rust
    fn main() {
        let m1 = String::from("Hello");
        let m2 = String::from("world");
        greet(&m1, &m2); // note the ampersands
        let s = format!("{} {}", m1, m2);
    }
    
    fn greet(g1: &String, g2: &String) { // note the ampersands
        println!("{} {}!", g1, g2);
    }
    ```
### Dereferences a Pointer Accesses Its Data
- If `&x` reference to `x`, then `*&x` allows you to read `x` directly.
- If `&x` is anya pointing to a variable `x`, then `*&x` is anya walking to a variable directly.
    ```rust
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    
    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);
    
    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
    ```

- A `&x` operator only has Read access to x; A `&mut x` operator has both Read and Write access to `x`
    ```rust
    let x = Box::new(1);
    let x_point: &Box<i32> = &x;
    **x_point += 1;  // ERR, reference no write access

    let x_point_mut: &mut Box<i32> = &mut x;
    **x_point_mut += 1  // Can be compiled
    ```

## Rust Avoid Simultaneous Aliasing and Mutation

- Creating a reference to a variable is Aliasing. Updating a variable is Mutation. When combined, it's a recipe for disaster:
    - By deallocating the aliased data, leaving the other variable to point to deallocated memory.
    - By mutating the aliased data, invalidating runtime properties expected by the other variable.
    - By concurrently mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.
    
    ```rust
    // The code below is not compillable
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4); // .push() mutate, allocate v to a different memory location as vector expanded
    println!("Third element is {}", *num);  // num now points to nothing
    ```
> **Pointer Safety Principle**: data should never be aliased and mutated at the same time.

- Enforcing the *Pointer Safety Principle*, a borrow checker is used to looks for potentially unsafe operations involving references.

    ```rust
    let mut v: Vec<i32> = vec![1, 2, 3];  // v: RWO
    let num: &i32 = &v[2];  // v:   R__
                            // num: R_O
                            // *num:R__
    v.push(4);  // ERR: W not in v
    println!("Third element is {}", *num);
    ```
    ```bash
    error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
     --> test.rs:4:1
      |
    3 | let num: &i32 = &v[2];
      |                  - immutable borrow occurs here
    4 | v.push(4);
      | ^^^^^^^^^ mutable borrow occurs here
    5 | println!("Third element is {}", *num);
      |                                 ---- immutable borrow later used here
    ```
- Core idea behind borrow checker is each variable has three kinds of permissions:
    - **Read (R)**: Data can be copied to another location
    - **Write (W)**: Data can be mutated in-place
    - **Own (O)**: Data can be moved or dropped

    ```rust
    let mut v: Vec<i32> = vec![1,2,3];          // v:   RWO
    
    let num: &i32 = &v[2];                      // v:   R__
                                                // num: R_O (borrowed)
                                                //*num: R__
    
    println!("Third element is {}", *num);      // v:   RWO (returned)
                                                // num: ___
                                                //*num: ___

    v.push(4);                                  // v: ___ (returned EOF)
    ```
 - Permissions are returned at the end of a reference's lifetime (i.e. last time reference is used)
