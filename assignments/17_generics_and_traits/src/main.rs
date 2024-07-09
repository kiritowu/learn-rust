use std::fmt;

struct Bar {
    boo: String,
}

// Define custom trait
trait MyTrait {
    fn boo1(&self) -> String;
}

// Implement a trait for a custom Type
impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bar(boo={})", self.boo)
    }
}

// Implement custom trait
impl MyTrait for Bar {
    fn boo1(&self) -> String {
        format!("{}1", self.boo)
    }
}

fn main() {
    println!("Hello, world!");
    let bar1 = Bar {
        boo: String::from("helo"),
    };
    println!("{bar1}");
    println!("{}", bar1.boo1());
}
