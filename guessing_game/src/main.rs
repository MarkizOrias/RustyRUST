use std::io;

fn main() {
    println!("Welcome to the guessing game!\nPick a number from 1 to 100!");
    // division of lines is possible with \n

    let mut guess = String::new();
    // defining by 'let' a mutable variable 'guess'
    // bound (=) to the new() is associated function of a string type

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    // calling a function of standard input
    // reading the entry and assigning entered value to a guess mutable variable (of a String type)
    // error handling, printing error message

    println!("Thank you!\nYou entered {guess}");
    // entering an object to printed line with {} brackets
}
