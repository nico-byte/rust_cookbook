use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // mut will make the variable mutable, so we can change it later.

        io::stdin()
            .read_line(&mut guess)      // & argument is a reference to the variable, so we don't need to copy it.
            .expect("Failed to read line");
            // returns an enum (enums can be in one of many possible states - variants)

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ will catch any error, so the second arm's code - continue - will run.
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            // Ordering type has 3 possible outcomes/variants: Less, Greater, Equal
        }
    }
}
