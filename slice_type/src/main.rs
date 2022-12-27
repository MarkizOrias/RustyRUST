// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// A slice is a kind of reference, so it does not have ownership.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

// The first_word function refers to the String type from the main function and returns a usize integer.
// Then we convert the word into the bytes type, loop through the string and return usize length of the word, by:
// Checking the tuple of index and reference to an item from the bytes array in the enumeration of iter method.
// If item is bound to b' ', return i.
// Otherwise, leturn the length of the word, as it does not contain any space.

// With slices, you can achieve another thing:

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// We create slices using a range within brackets by specifying [starting_index..ending_index],
// where starting_index is the first position in the slice and ending_index is one more than the last position
// in the slice.

// With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:

let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

// By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:

let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

// You can also drop both values to take a slice of the entire string. So these are equal:

let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];

// With all this information in mind, let’s rewrite first_word to return a slice. The type that signifies “string slice” is written as &str:
// Slices make this bug impossible and let us know we have a problem with our code much sooner.

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; 
// &str is an immutable reference.

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

// Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take 
// a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. 
// The println! after the call to clear uses the reference in word, so the immutable reference must still be active 
// at that point. Rust disallows the mutable reference in clear and the immutable reference in word from existing 
// at the same time, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated 
// an entire class of errors at compile time!


// Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:

let s = "Hello, world!";

// The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.
// Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:

fn first_word(s: &String) -> &str {
// A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.

fn first_word(s: &str) -> &str {
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}