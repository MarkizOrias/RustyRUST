fn main() {
    let mut x = 5;
    // if not mut: cannot assign twice to immutable variable `x`
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
