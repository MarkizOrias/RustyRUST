// Definition: Ownership is a set of rules that govern how a Rust program manages memory.
// In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks.

// Stack and Heap principles:

// Stack stores a value in order and removes the values in the opposite order - LIFO.
// Ex. pile of plates - adding or removing is processed only on the top plate in the pile.
// Adding: pushing onto the stack (not considered as allocation), removing: popping off the stack.

// Storing data with unknown size or mutable is processed on the heap.
// In the heap, memory allocator finds a place in the memory, where it can allocated the value, based on its size.
// When stored, memory allocator returns a pointer, which is the address of the location on the memory.
// This process is called allocation on the heap. Because the pointer is know size, it can be stored on the stack.
// But the actual data is stored on the address, stored by a pointer.
// Real life analogy: you come with the group of x people (data), waiter (memory allocator) finds you a table (address).
// If someone is late (data considered in x number of people), waiter (memory allocator)
// can find you using the pointer.

// Stack is quicker than heap, as data on stack is located near eachother, while heap is spread across.
// This generates slowliness of processing (like getting orders from all the tables in restaurant).

// Rules of ownership in Rust:
// // Each value in Rust has an owner.
// // There can only be one owner at a time.
// // When the owner goes out of scope, the value will be dropped.

// Variable scope - range within which a variable is valid:

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

// The types covered previously are of a known size, can be stored on the stack 
// and popped off the stack when their scope is over, and can be quickly 
// and trivially copied to make a new, independent instance if another part of code needs 
// to use the same value in a different scope. But we want to look at data that is stored 
// on the heap and explore how Rust knows when to clean up that data, and the String type 
// is a great example.

// String literals are know size, because we define them in the code, like in the above example.
// In the case of a string literal, we know the contents at compile time, so the text 
// is hardcoded directly into the final executable. This is why string literals are fast and efficient. 
// For unknown size (like the one that stores user's input), we have a String type.
// We can create a String from a string literal, using the from function:

let s = String::from("hello");

// String can be mutated:

let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`

// The memory must be requested from the memory allocator at runtime - during assignment, e.g. String::new() or String::from("").
// We need a way of returning this memory to the allocator when we’re done with our String.

// However, the second part is different. In languages with a garbage collector (GC), 
// the GC keeps track of and cleans up memory that isn’t being used anymore, 
// and we don’t need to think about it. In most languages without a GC, 
// it’s our responsibility to identify when memory is no longer being used 
// and to call code to explicitly free it, just as we did to request it. 
// Doing this correctly has historically been a difficult programming problem. 
// If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. 
// If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

// Rust takes a different path: the memory is automatically returned once the variable 
// that owns it goes out of scope. Here’s a version of our scope example using a String instead of a string literal:

{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
// There is a natural point at which we can return the memory our String needs to the allocator: 
// when s goes out of scope. When a variable goes out of scope, Rust calls a special function 
// for us. This function is called drop, and it’s where the author of String can put the code 
// to return the memory. Rust calls drop automatically at the closing curly bracket.


// Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop 
// function and cleans up the heap memory for that variable. But Figure 4-2 shows both data 
// pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, 
// they will both try to free the same memory. This is known as a double free error 
// and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to 
// memory corruption, which can potentially lead to security vulnerabilities.

// To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
// Therefore, Rust doesn’t need to free anything when s1 goes out of scope. 
// Check out what happens when you try to use s1 after s2 is created; it won’t work:

let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);

// In this example, s2 copies the content of s1 variable: it's pointers, length and capacity.
// s1 becomes known as "move", hence calling s1 will result in runtime error.

// If we do want to deeply copy the heap data of the String, not just the stack data, 
// we can use a common method called clone. We’ll discuss method syntax in Chapter 5, 
// but because methods are a common feature in many programming languages, you’ve probably seen them before.

// Here’s an example of the clone method in action:

let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);

// Here are some of the types that implement Copy:

// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy. 
// For example, (i32, i32) implements Copy, but (i32, String) does not.
                                  
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.