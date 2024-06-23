# Enum

- Enum group data of similar attributes together, without enforcing similar
  structure and datatype.
- Accessing an enum uses `::` operator. And parse as argument type in function, just like other datatypes. (e.g. `fn route(ip_kind: IpAddrKind)`)
	```rust
	enum IpAddrKind {
		V4,
		V6,
	}

	fn main() {
		let four = IpAddrKind::V4;
		let six = IpAddrKind::V6;
	}
	```
- Other than grouping namespaces or keys, rust's enums allows putting data (of different type: strings, numeric, strucs or tuples) directly into each enum variant.
	```
	enum Message {
	    Quit,
	    Move { x: i32, y: i32 },
	    Write(String),
	    ChangeColor(i32, i32, i32),
	}

	enum IpAddrKind {
		V4(u8, u8, u8, u8),
		V6(String),
	}

	fn main() {
		let four = IpAddrKind::V4(127, 0, 0, 1);
		let six = IpAddrKind::V6(String::from("..1"));
	}
	```
- Finally, just like `struct`, `enum` allows methods to be `impl` on a particular enum.
	```rust
	impl Message {
	    fn call(&self) {
	        // method body would be defined here
	    }
	}
	```

# Option<T>

- `Option<T>` is a enum defined by standard library, denoting an Optional scenario for a data.
	- Rust's emphasis of safety makes implementation of "null" values more stringent, using Enum to denote possible scenario of a value being present or absent.
		```rust
		enum Option<T> {
			None,
			Some(T),
		}
		```
	- Implementation of `enum` with `Some` prevents unexpected bahaviour by forcing developer to always having to address the null-case before unwrapping actual data out of `Some`. This brings us to `match` expression as alternative to usual `if-else` control flow construct.

# Match Control Flow Construct

- `match` allows compare a value against a series of patterns and then execute code based on pattern matches.
	- Patterns can be literal values, variable names, wildcards and many other things.
- `match` returns at the first pattern that it fits. And can be used as expression.
	
	```rust
	#[derive(Debug)] // so we can inspect the state in a minute
	enum UsState {
	    Alabama,
	    Alaska,
	    // --snip--
	}
	
	enum Coin {
	    Penny,
	    Nickel,
	    Dime,
	    Quarter(UsState),
	}
	
	fn value_in_cents(coin: Coin) -> u8 {
	    match coin {
	        Coin::Penny => {
			println!("Lucky penny");
			1
		},
	        Coin::Nickel => 5,
	        Coin::Dime => 10,
		Coin::Quarter(state) => {
        	    println!("State quarter from {:?}!", state);
        	    25
        	}
	    }
	}
	```

### Matching with Option<T>

- Snippets below shows how `match` is useful with `Option<T>`:
    ```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), 
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    ```
### Match Are Exhaustive

- `match` catches the issue where some patterns are not covered, preventing unexpected behaviour.
- `_` or other variable placeholder can be used to covered un-named cases in `match`

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

# Concise Control Flow with `if let`

- The example below shows control flow using `match`, concern only when value is `Some`, ignoring the `None` case. This requires a boilerplate `_ => ()` which could be combersome for some.
	```rust
	let config_max = Some(3u8);
	match config_max {
		Some(max) => println!("{}", max),
		_ => (),
	}
	```

- `if let` provide a concise way for pattern-matching of a single expression, with less typing, indentation and boilerplate code; losing exhausitive checking that `match` enforces.
	```rust
	let config_max = Some(3u8);
    	if let Some(max) = config_max {
    	    println!("The maximum is configured to be {}", max);
    	}
	```

