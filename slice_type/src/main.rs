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
