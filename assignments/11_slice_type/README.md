# Slice Type

- *Slices* is a reference to a contiguous sequence of elements in a collection rather than the whole collection.
- Slice is a "fat" kind of reference (i.e. non-owning pointer), that dont just point to the String but element and length of it.
```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];  // Slices denoted with &str
let world: &str = &s[6..];  // Slices till end of string (equv -1)
let all: &str = &s[..];     // Slice of whole string
let s2: &String = &s;       // Normal references

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```
