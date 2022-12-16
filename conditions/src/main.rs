fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// Condition must be a bool. Expressions are divided into arms.
// Structured conditions are easy to code by else if. If many conditions are there, you can use match, to assign a result
// To the tested condition.

fn main() {
    loop {
        println!("again!");
    }
}

// The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// We are are able to stop loops by a break expression.

fn main() {
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
}

// This loop prints:
// count = 0
// remaining = 10
// remaining = 9
// count = 1
// remaining = 10
// remaining = 9
// count = 2
// remaining = 10
// End count = 2

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// Looping through an array (collection)
// However, this approach is error prone; we could cause the program to panic if the index value or test condition
// is incorrect. For example, if you changed the definition of the a array to have four elements but forgot
// to update the condition to while index < 4, the code would panic. It’s also slow, because the compiler adds runtime code
// to perform the conditional check of whether the index is within the bounds of the array on every iteration
// through the loop.

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
// Even in situations in which you want to run some code a certain number of times, as in the countdown example
// that used a while loop in Listing 3-3, most Rustaceans would use a for loop. The way to do that would be to use a Range,
// provided by the standard library, which generates all numbers in sequence starting from one number
// and ending before another number.

// Here’s what the countdown would look like using a for loop and another method we’ve not yet talked about, rev,
// to reverse the range:

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
