// Instead of moving the pointer across multiple fuctions / locations in the code,
// We can use a reference: '&'
// So the previous example of changing variables ownership, we could write:

fn main() {
    let s: String = String::from("MakaPaka");
    let len: usize = count_len(&s);

    println!("{s} string has {len} length");
}

fn count_len(s: &String) -> usize {
    s.len()
}

// instead of:

fn main() {
    let s: String = String::from("Makapaka");
    let (s, len) = count_len(s);
    println!("{s} string has {len} length");
}

fn count_len(s: String) -> (String, usize) {
    let len: usize = s.len();
    (s, len)
}

// Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

// The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used.
// Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference. Let’s add some explanatory annotations:

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// The scope in which the variable s is valid is the same as any function parameter’s scope, but the value
// pointed to by the reference is not dropped when s stops being used, because s doesn’t have ownership.
// When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

// We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
// We can fix the code from Listing 4-6 to allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:

fn main() {
    let mut s: String = String::from("Hello");
    change_string(&mut s);

    println!("{s}");
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

// Mutable references have one big restriction: if you have a mutable reference to a value,
// you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

// This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. 
// The first mutable borrow is in r1 and must last until it’s used in the println!, but between 
// the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

// The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. 
// It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction 
// is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

// // Two or more pointers access the same data at the same time.
// // At least one of the pointers is being used to write to the data.
// // There’s no mechanism being used to synchronize access to the data

// We can use scopes to exclude the posibility of generating data races issue:

let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;

// Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

// We also cannot have a mutable reference while we have an immutable one to the same value.
// Users of an immutable reference don’t expect the value to suddenly change out from under them! 
// However, multiple immutable references are allowed because no one who is just reading the data 
// has the ability to affect anyone else’s reading of the data.

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);

// This is OK, as we first call the values in the print function 
// After that, data is no longer used by any other part of the code, 
// so we can mutate the value.

// In languages with pointers, it’s easy to erroneously create 
// a dangling pointer—a pointer that references a location 
// in memory that may have been given to someone else—
// by freeing some memory while preserving a pointer to that memory. 
// In Rust, by contrast, the compiler guarantees that references 
// will never be dangling references: if you have a reference to some data, 
// the compiler will ensure that the data will not go out of scope before 
// the reference to the data does.

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

//   Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. 
// But we tried to return a reference to it. That means this reference would be pointing to an invalid String. 
// That’s no good! Rust won’t let us do this.

// The solution here is to return the String directly:

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}