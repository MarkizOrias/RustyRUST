// Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.
// Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece
// of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples:
// you don’t have to rely on the order of the data to specify or access the values of an instance.

// To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe
// the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names
// and types of the pieces of data, which we call fields. For example, Listing 5-1 shows a struct that stores information
// about a user account.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

// To get a specific value from a struct, we use dot notation. For example, to access this user’s email address,
// we use user1.email. If the instance is mutable, we can change a value by using the dot notation and assigning
// into a particular field. Listing 5-3 shows how to change the value in the email field of a mutable User instance.

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

// Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
// As with any expression, we can construct a new instance of the struct as the last expression in the function body
// to implicitly return that new instance.

// We can create a user profile in the separate create function:

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// But there is a shorthand for that!
// In case we are bounding email parameters to the struct internal values, we can write the same fuction in the following way:

fn create_users_profile(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// In this case we ommited repeating parameter's name with it's value.

// It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using struct update syntax.

fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
// Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7.
// The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields
// in the given instance.

fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

// The code also creates an instance in user2 that has a different value for email but has the same values
// for the username, active, and sign_in_count fields from user1. The ..user1 must come last to specify
// that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify
// values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

// This is cool!!!
// Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have
// the added meaning the struct name provides but don’t have names associated with their fields;
// rather, they just have the types of the fields. Tuple structs are useful when you want to give
// the whole tuple a name and make the tuple a different type from other tuples, and when naming each
// field as in a regular struct would be verbose or redundant.

// To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple.
// For example, here we define and use two tuple structs named Color and Point:

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    let first_idex_black: i32 = black.0;
}

// A tuple struct instances are similar to tuples in that you can destructure them into their individual pieces,
// and you can use a . followed by the index to access an individual value.

// You can also define structs that don’t have any fields! These are called unit-like structs
// because they behave similarly to ()
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

// To define AlwaysEqual, we use the struct keyword, the name we want, then a semicolon.
// No need for curly brackets or parentheses! Then we can get an instance of AlwaysEqual in the subject variable
// in a similar way: using the name we defined, without any curly brackets or parentheses. Imagine that later
// we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance
// of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement
// that behavior!

// We used the owned String type rather than the &str string slice type. This is a deliberate choice because we want
// each instance of this struct to own all of its data and for that data to be valid for as long
// as the entire struct is valid.

// It’s also possible for structs to store references to data owned by something else, but to do so requires
// the use of lifetimes.

// The following code will not work:

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
