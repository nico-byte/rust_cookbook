fn main() {
    // Variables are immutable by default
    // with mut keyword variables can be mutated
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x in the outer scope is {x}");

    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {guess}");

    let x = 2.0; // f64
    let x: f32 = 3.0; // f32
    print!("The value of x is {x}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;

    let f: bool = false;

    // Tuples
    // can hold multiple data types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // access certain index of  tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Arrays
    // fixed size and one data type
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", 
                              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // initialize array with 5 i32s
    let a = [3; 5]; // initialize array with 3 repeated 5 times

    // accessing array elements
    let first = a[0];
    let second = a[1];
    
    // functions and closures
    let sum = plus_one(first);
    println!("The value of sum is: {sum}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let fib = fibonacci(19);
    println!("The value of fib is: {fib}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn fibonacci(n: u32) -> u32 {
    let mut num1: u32 = 0;
    let mut num2: u32 = 1;
    let mut next_num: u32 = num2;
    let mut count: u32 = 1;

    while count < n-1 {
        count += 1;
        num1 = num2;
        num2 = next_num;
        next_num = num1 + num2;
    }

    next_num
}
