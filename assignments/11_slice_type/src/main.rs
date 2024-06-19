fn main() {
    let hello_world = String::from("Hello world");
    let len = hello_world.len();
    assert_eq!(hello_world[0..len], hello_world[..]);

    for c in hello_world.as_bytes().iter() {
        println!("{}", *c as char)
    }
}

