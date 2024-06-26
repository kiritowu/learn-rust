mod neighbour;
pub mod module;

fn main() {
    println!("Hello, world!");
    println!("{}", neighbour::hello_neighbour());
    module::a::hello_a();
    module::b::hello_b();
}
