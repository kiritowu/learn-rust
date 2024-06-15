fn main() {
    println!("Hello, world!");
    another_print_function(42, 'b');

    let five = five(); // Shadowing works on function too
    println!("Value of five is: {five}");

    let x_plus_1 = plus_1(1);
    println!("x_plus_1 is: {x_plus_1}");

    let var = statement_after_expression();
    assert!(var == {});
}

fn another_print_function(x: i32, char_label: char) {
    println!("Printing another statement with parameter: {x} {char_label}.");
}

fn five() -> i32 {
    5 // NOTE: expression dont have semicolon
}

fn plus_1(x: i32) -> i32 {
    x+1
}

fn statement_after_expression() -> () {
    // Result in error complaining expression does not have semicolon
    // 5
    5;
    println!("I am a statement");
    // Return expression is unit (i.e. `()`) instead
}
