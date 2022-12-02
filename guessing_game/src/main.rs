use rand::Rng;
// importing crate and range function - NOTE!!! dependency in cargo.toml file
// importing from crates.io the latest version => run 'cargo build'
// 'cargo update' updates the semantic version identifier in the dependencies
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!\nPick a number from 1 to 100!");
    // division of lines is possible with \n

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // defining secret_number as a random number (within the specified range) generatingcargo doc --open method
    // of a thread_rng function associated with rand crate
    // run 'cargo doc --open' to read documentation on the functions / methods

    // println!("Thank you!\nYou entered {guess}Random{secret_number}");
    // // entering an object to printed line with {} brackets

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        // defining by 'let' a mutable variable 'guess'
        // bound (=) to the new() is associated function of a string type

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        // calling a function of standard input, imported from the standard library
        // reading the entry and assigning entered value to a guess mutable variable (of a String type)
        // & - reference; read_line returns a Result -> enumeration, that has states (variants)
        // error handling, printing error message

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // casting guess into the unsigned 32-bit with error handler 'parse()'
        // trim is removing from string all \n\r -> enters and spaces
        // if OK variant
        // '_' captures any given symbol provided by the user, to return back to the beginning

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low!"),
            Ordering::Greater => println!("Your guess is too big!"),
            Ordering::Equal => {
                println!("You've guessed the number, bravo!!!");
                break;
            }
        }
    }
}
