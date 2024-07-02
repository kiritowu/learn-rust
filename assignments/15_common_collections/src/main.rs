use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    learn_vector();
    learn_string();
    learn_hashmap();
}

fn learn_vector() {
    println!("Learning Vector");
    // Create
    let mut v: Vec<i32> = Vec::new();
    for i in 0..3 {
        v.push(i);
    }
    let mut v = vec![1, 2, 3];

    // Read
    let last: &i32 = &v[v.len() - 1];
    println!("{}", last);
    let forth: Option<&i32> = v.get(3);
    match forth {
        Some(forth) => println!("The third element is {forth}"),
        None => println!("There is no third element."),
    }

    // Update using mutable reference
    for n_ref in &mut v {
        *n_ref = *n_ref + 1;
        println!("{n_ref}")
    }

    // Remove
    let last_ele = v.pop();
    match last_ele {
        Some(last) => println!("The first element is {last}"),
        None => println!("There is no first element"),
    }

    // NOTE: Using Enum to Store Multiple Types
    // As Vector allocates same size to each element, this requires every
    // element to have the same data-type.
    // To circumvent this, oen can use Enum with differrent data to store
    // data of different type in an enum.
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(0.2),
        SpreadSheetCell::Text(String::from("Hello")),
    ];
    for cell in &row {
        println!("{:?}", cell);
        match cell {
            SpreadSheetCell::Int(val) => println!("Integer: {val}"),
            SpreadSheetCell::Float(val) => println!("Float: {val}"),
            SpreadSheetCell::Text(val) => println!("Text: {val}"),
        }
    }
}
fn learn_string() {
    println!("Learning String");
    // Create
    let _s = String::from("building string from literal");

    // Read
    // WARNING: In Rust, slicing of string is cumbersome:
    // String is a wrapper over Vec<u8>
    // But Unicode some scalar value//char to be formed with multiple bytes
    // And in some language, multiple scalar value is used to form Grapheme Clusters
    //
    // However, rust allows slicing of string using a range of bytes, `.chars()` and `.bytes()`
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Ok
    println!("{s}");
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Update
    // NOTE: Use `.push_str()`, `+` or `format!`
    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

fn learn_hashmap() {
    println!("Learning Hashmap");

    // Create
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Read
    // Using iter or `.get`
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Update
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.remove("Blue");
    if let None = scores.get("Blue") {
        println!("Blue is not found in hashmap");
    }
}
