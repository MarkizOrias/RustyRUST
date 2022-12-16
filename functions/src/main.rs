// ***---FUNCTIONS---***

// We can call any function we’ve defined by entering its name followed
// by a set of parentheses. Because another_function is defined in the program,
// it can be called from inside the main function. Note that we defined another_function
// after the main function in the source code; we could have defined it before as well.
// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope
// that can be seen by the caller.

// We can define functions to have parameters, which are special variables that are part of a function’s signature.
// When a function has parameters, you can provide it with concrete values for those parameters.
// Technically, the concrete values are called arguments, but in casual conversation,
// people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition
// or the concrete values passed in when you call a function.

// In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design:
// requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere
// in the code to figure out what type you mean. The compiler is also able to give more helpful error messages
// if it knows what types the function expects.

fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// ***---STATEMENTS & EXPRESSIONS---***

// Defining functions (or variables) is a statement. Statements do not return values, expressions do.
// Expressions like x = y = 6 return an error in rust, not like in C or Ruby, because let x: i32 = (let y: i32 = 6);
// expression do not bound any value to x.

// Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6;
// is an expression that evaluates to the value 6. Calling a function is an expression. Calling a macro is an expression.
// A new scope block created with curly brackets is an expression

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Inner scope is a block that evaluates to 4.
// That value gets bound to y as part of the let statement. Note that the x + 1 line doesn’t have a semicolon at the end,
// unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons.
// If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
// Keep this in mind as you explore function return values and expressions next.

// ***---FUNCTIONS WITH RETURN VALUE---***

// Functions can return values to the code that calls them. We don’t name return values,
// but we must declare their type after an arrow (->). In Rust, the return value of the function
// is synonymous with the value of the final expression in the block of the body of a function.
// You can return early from a function by using the return keyword and specifying a value,
// but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

// OR

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
