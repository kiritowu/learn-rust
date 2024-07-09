# Generic Datatype, Traits

## Generics

- Denoted by Uppercase single letter (e.g. `T`, `U`), it represents a generic type to be filled up on compile time.
- Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. In this process, the compiler does the opposite of the steps we used to create the generic function  the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

```rust
// Generic in Enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generics in Struct
struct Point<T, U> {
    x: T,
    y: U,
}

// Generic in methods
impl<T, U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## Traits

- Traits define implementation of custom method to a Type.
- A trait is implemented using `impl` Trait `for` Type syntax.
- Default implementation can be defined when a trait is first defined.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        // Default behavior
        String::from("(Read more...)")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

### Traits as Parameters
- Trait can be used as parameter in method declaration

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bound with + Syntax
pub fn notify<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```
