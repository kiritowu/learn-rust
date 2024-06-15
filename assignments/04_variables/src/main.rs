fn main() {
    // Variable is immutable by default
    // let x = 5;  // Compile Error
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constant named with uppercase with underscore as seperator
    // type declaration is compulsary
    // constant can be declared as expression
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of constant is {THREE_HOURS_IN_SECONDS}");

    // Shadowing of variable creates transformation of the variable that is immutable(if it was
    // immutable) with different datatype
    let y = 5;
    let y = y + 1; // Shadow y with different value
    {
        let y = y * 2;
        println!("The value of y in inner scope: {y}");
    }
    println!("The value of y in outer scope: {y}");

    let spaces = "   ";
    println!("Spaces before shadowing: {spaces}");
    let spaces = spaces.len(); // No compile error with different datatype
    // spaces = spaces.len() // Compile error
    println!("Spaces after shadowing: {spaces}");
}
