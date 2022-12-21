// use std::io;

// fn main() {
//     println!("Hi! \nPlease type in which option would you like to use:\n1: Farenheit to Celsius\n2: Celsius to Farenheit");

//     loop {
//         print!("Your choice: ");
//         io::stdout().flush().unwrap();

//         let mut entry: String = String::new();

//         io::stdin()
//             .read_line(&mut entry)
//             .expect("Failed to read line");

//         let entry: i32 = match entry.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 print!("Given entry: {entry} is not a number!\nPlease enter again!");
//                 continue;
//             }
//         };
//     }
// }
use std::io;

fn rdln(entry: &mut String) {
    io::stdin().read_line(entry).expect("Cannot read!");
}

fn 