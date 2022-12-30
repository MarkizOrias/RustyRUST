// A variable approach:
fn main() {
    let width: u32 = 20;
    let length: u32 = 30;

    println!("The area of a rectangle: {}", calculate_area(width, length));
}

fn calculate_area(w: u32, l: u32) -> u32 {
    w * l
}

// A tuple approach - lack of clarity which dimension stands for what:
fn main() {
    let dimensions: (u32, u32) = (20, 30);
    println!("The area of a rectangle: {}", area(dimensions));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// A struct approach - adding more meaning:
struct Dimensions {
    width: u32,
    length: u32,
}

fn main() {
    let dim: Dimensions = Dimensions {
        width: 20,
        length: 30,
    };
    println!("The area of a rectangle: {}", area(&dim));
}

fn area(dimsn: &Dimensions) -> u32 {
    dimsn.width * dimsn.length
}

// Our area function is now defined with one parameter, which we’ve named dimsn, whose type is
// an immutable borrow of a struct Dimensions instance. As mentioned in Chapter 4, we want to borrow the
// struct rather than take ownership of it. This way, main retains its ownership and can continue using dim,
// which is the reason we use the & in the function signature and where we call the function.

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}

// The above code will not be compiled, having an error with println! macro. This happens, because rust doesn't know
// in which format we want the data from struct to be printed. It suggests using: {:?} (or {:#?} for pretty-print)

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// Another way to print out a value using the Debug format is to use the dbg! macro,
// which takes ownership of an expression (as opposed to println! that takes a reference),
// prints the file and line number of where that dbg! macro call occurs in your code along with the resulting value
// of that expression, and returns ownership of the value.

// We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value,
// the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg!
// to take ownership of rect1, so we use a reference to rect1 in the next call.

// We can see the first bit of output came from src/main.rs line 10, where we’re debugging the expression 30 * scale,
// and its resulting value is 60 (the Debug formatting implemented for integers is to print only their value).
// The dbg! call on line 14 of src/main.rs outputs the value of &rect1, which is the Rectangle struct.
// This output uses the pretty Debug formatting of the Rectangle type. The dbg! macro can be really helpful
// when you’re trying to figure out what your code is doing!
