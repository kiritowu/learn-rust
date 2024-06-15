fn main() {
    let number = 3;

    if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} not divisible by 2 or 3");
    }

    // if number { } // Throw error as number is not bool

    let number = if 1 > 2 { 1 } else { 2 };
    // let number = if 1 > 2 { 1 } else { '2' }; // Error of type ambiguity
    println!("If statement can be used with expression of same type {number}");

    // if 1 { // Error
        // println!("1 is a boolean");
    // }

    // if 0b1111_1111 { // Error
        // println!("true byte is a boolean");
    // }


    // Loops
    let mut x = 0;
    let loop_return = loop {  // infinite loop
        println!("x is {x}");
        if x == 5 {
            break x; // Break will return expression
        }
        x += 1;
    };
    assert!(x == 5);
    assert!(loop_return == 5);


    let mut i = 0;
    'outer_loop: loop { // Declare loop labels
        let mut j = 0;
        loop {
            j += 1;
            if j == 10 {
                println!("i: {i}, j: {j}");
                break; // Break inner loop
            }
            if i == 10 {
                println!("break outer");
                break 'outer_loop; // Break with loop labels
            }
        }
        i += 1;
    };

    let mut j = 0;
    while j < 10 {  // While loop
        j += 1;
    }
    println!("j: {j}");

    let range_exclude_end = 0..5;  // Declare a Range from 0->4, exclude end
    let range_include_end = 0..=5;  // Declare a Range from 0->5, include end
    for i in range_exclude_end {
        println!("i: {i}");
    }
    for i in range_include_end {
        println!("i: {i}");
    }
    for rev_i in (0..=5).rev() {
        println!("rev_i: {rev_i}");
    }

    let temp = farenheit_to_celsius(123.0);
    println!("123 F is {temp} C");

    let fib_n = fibonacci(10);
    // let fib_n = fibonacci_dp(10);
    println!("fibonaci(10) is {fib_n}");

    let day: usize = 1;
    println!("Printing twelve days of christmas with day: {day}");
    println!("=================================================");
    print_twelve_days_of_christmas(day);
}

fn farenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0)*5.0/9.0
}

fn fibonacci(n: u32) -> u32 {
    // Generate fibonaci with recursion with O(2^n)
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

fn fibonacci_dp(n: usize) -> usize {
    let mut memory = vec![0; n+1];
    memory[0] = 0;
    memory[1] = 1;
    
    for i in 2..=n {
        memory[i] = memory[i-1] + memory[i-2];
    }

    memory[n]
}

fn print_twelve_days_of_christmas(day: usize) {
    // Ensure day is 1..=12
    if day == 0 {
        panic!("day must be greater than 0");
    }
    if day > 12 {
        panic!("days should be less than 12");
    }

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-miling",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let days = [
        "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine", "ten", "eleven", "twelve"
    ];

    println!("On the {} day of Christmas my true love sent to me", days[day-1]);
    for i in (1..day).rev() {
        println!("{},", gifts[i]);
    }
    if day > 1 {
        println!("And {}.", gifts[0]);
    } else {
        println!("{}", gifts[0]);
    }
}
