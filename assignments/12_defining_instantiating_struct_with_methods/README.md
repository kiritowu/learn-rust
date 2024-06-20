# Defining and Instantiating Structs

- Struct grouped multiple named/unnamed values of different type together.
- Struct does not allow only marking only certain fields as mutable but all as same mutability state.
- Field init shorthand is available if key and name of the variable are the same.
- Struct update syntax `..` allows updating a struct values using other struct, transfering the ownership in the process.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail.example.com");
}
```

- Tupple struct is type of struct that dont have name associated to field but just the types.
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
- Unit stuct is a struct without any key or values, behave similarly to `()`
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## Methods in Struct

- Methods are like functions, declared with `fn`, defined defined within the context of a struct (or an enum or a trait object).
- Method can take the same name as attribute.
- Method can accept `&self` (read only reference), `&mut self` (mutable reference) and `self` (owned) respectively, depending on use case.
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    // multiple impl block is allowed
   impl Rectangle {    
        fn area(&self) -> u32 {
            self.width * self.height
        }
    
        fn set_width(&mut self, width: u32) {
            self.width = width;
        }
    
        fn max(self, other: Rectangle) -> Rectangle {
            Rectangle { 
                width: self.width.max(other.width),
                height: self.height.max(other.height),
            }
        }
    } 
    
    fn main() {
        let rect1 = Rectangle{
            width: 30,
            height: 50,
        };
    
        let area = rect1.area();
    }
    ```

### Associated Function

- Associated function is a implemented function that does not take `&self` as parameter.
- Associated function often used for constructors that return a new instance of struct, often called `new`.
- Called using `::` syntax with struct name. (e.g. `Rectangle::new()`)
    ```rust
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    
    fn main() {
        let square_rect = Rectangle::square(5);
    }
    ```
