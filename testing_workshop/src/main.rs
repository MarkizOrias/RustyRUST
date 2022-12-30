// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("user1.name"),
//         email: String::from("first_example@email.com"),
//         sign_in_count: 1,
//     };

//     let user2: User = User {
//         username: String::from("user2.name"),
//         email: String::from("another_example@email.com"),
//         ..user1
//     };

//     // Note for the ownership - we can still use values of the active parameter and sign_in_count, as these follow under the Copy trait.
//     // Let's check the debug printing.
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black: Color = Color(7, 0, 0);
    let _initial_position: Point = Point(0, 0, 0);

    let first_idex_black: i32 = black.0;

    println!("First index of black is: {}", first_idex_black);
}
