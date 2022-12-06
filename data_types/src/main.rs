fn main() {
    // ***-----SCALAR-----***

    // Rust is a statically typed language - a type of each variable has to be defined, rust can read a type based on the input
    // A scalar type represents a single value
    // Scalars divide into 4 subtypes: integers, floating-point numbers, booleans and characters.

    // ***INTEGERS***

    // Can be signed or unsigned, the formula is: from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive
    // From u/i8 to u/i128; default is i32
    // i/usize integers have a size of an architecture, e.g. 64bit OS would have usize equal to uint64
    // you can use this symbol as a separator between k's - 1_000_000
    // integers can be: decimal, hex, octal, binary, byte

    // OVERFLOWS - result in panicing, but in --release flag, rust performs two's complement wrapping, for u8 256 becomes 0, 257 becomes 1 etc.
    // To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:
    // Wrap in all modes with the wrapping_* methods, such as wrapping_add
    // Return the None value if there is overflow with the checked_* methods
    // Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    // Saturate at the value’s minimum or maximum values with saturating_* methods

    // ***FLOATING-POINTS***

    // Only 2 types:
    // double precision (default one)
    let x = 2.0; // f64

    // single float precision
    let y: f32 = 3.0; // f32

    // ***NUMERIC OPERATORS***

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // ***BOOLEANS***

    // Booleans are 1 byte in size.
    let t = true;
    let f: bool = false; // with explicit type annotation

    // ***CHARACTER TYPES***

    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
    // which means it can represent a lot more than just ASCII. Accented letters;
    // Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    // However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is
    // may not match up with what a char is in Rust.

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // ***-----COMPOUND-----***

    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // ***TUPLES***
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    // For example:

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // The tuple without any values has a special name, unit. This value and its corresponding type are both written ()
    // and represent an empty value or an empty return type. Expressions implicitly return the unit value
    // if they don’t return any other value.

    let f: () = ();

    // ***ARRAYS***

    // Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages,
    // arrays in Rust have a fixed length.

    // Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure
    // you always have a fixed number of elements. An array isn’t as flexible as the vector type, though.
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    // If you’re unsure whether to use an array or a vector, chances are you should use a vector.

    // However, arrays are more useful when you know the number of elements will not need to change.
    // For example, if you were using the names of the month in a program, you would probably use an array
    // rather than a vector because you know it will always contain 12 elements:

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // You write an array’s type using square brackets with the type of each element, a semicolon,
    // and then the number of elements in the array, like so:

    let a = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

    let a = [3; 5];

    // The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing
    // let a = [3, 3, 3, 3, 3]; but in a more concise way.

    // Accessing the array element by array's name and index of an element, e.g. a[1] => 2nd element
}
