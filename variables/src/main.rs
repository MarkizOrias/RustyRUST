fn main() {
    // VARIABLES

    let mut x = 5;
    // if not mut: cannot assign twice to immutable variable `x`
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // CONSTANTS

    // constants are immutable by default and that state cannot be changed
    // used to bound a name (in capital letters) in order to save some fixed value

    const SECONDS_IN_THREE_HOURS: u16 = 60 * 60 * 3;

    println!("The number of seconds in 3 hours is: {SECONDS_IN_THREE_HOURS}");

    // SHADOWING

    let y = 5;
    // initial definition of y, shadowed further down the code

    let y = y + 1;
    // y is overshadowing the first definition

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    // re-defining 'spaces' variable with different type of the variable returns no compile time error

    println!("The number of spaces is: {spaces}");
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // // above returns compile time error due to mismatched type
}
