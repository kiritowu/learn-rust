use std::io;

fn main() {
    let uint8: u8 = 255;
    // let uint8 = 256; // Integer overflow error
    // let uint8 = -123; // Unsigned value cannot be negated
    println!("uint8: {uint8}");

    // Integer can be declared as literal with different number base
    let hex16: u8 = 0x10;
    let binary16 = 0b0001_0000;
    let byte_a: u8 = b'A';
    println!("hex16: {hex16}");
    println!("binary16: {binary16}");
    println!("byte_a: {byte_a}");

    // Floating Point
    let float64: f64 = 0.1;
    let float32: f32 = 0.1;
    // let float_arch: fsize = 0.1; // No such thing as fsize, only usize
    println!("{float64}, {float32}");

    // Character
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';  // Unicode character
    println!("{c}, {z}, {heart_eyed_cat}");

    // Tuple
    let tup: (f32, f64, u8) = (0.2, 0.3, 255);
    let (a,b,c) = tup;
    let a0 = tup.0;
    println!("tup = ({a},{b},{c}), {a0}");

    // Array
    let arr123: [i32; 3] = [1, 2, 3];
    let null5 = [0; 5];
    let arr0 = arr123[0];
    let null1 = null5[1];
    println!("{arr0}, {null1}");

    // Try invalid index

    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readline");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of element at index {index} is: {element}");
    
}
