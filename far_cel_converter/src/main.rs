use std::io::{self, Write};

fn rd_line(s: &mut String) {
    io::stdin().read_line(s).expect("Cannot read STDIN");
}

fn calc_celsius(y: f64) -> f64 {
    (y - 32.0) * (5.0 / 9.0)
}

fn calc_farenheit(y: f64) -> f64 {
    y * (9.0 / 5.0) + 32.0
}

fn main() {
    print!("Welcome to Farenheit/Celsius converter!\n1. Convert Farenheit to Celsius.\n2. Convert Celsius to Farenheit.\nPlease choose your option: ");
    io::stdout().flush().expect("Failed to flush!");
    loop {
        let mut x = String::new();
        rd_line(&mut x);
        let x: i32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Given entry is not a number!\nPlease choose your option: ");
                io::stdout().flush().expect("Failed to flush!");
                continue;
            }
        };

        if x == 1 {
            let mut y = String::new();
            print!("Enter temperature in Farenheit: ");
            io::stdout().flush().expect("Failed to flush!");
            rd_line(&mut y);
            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Given entry is not a number!");
                    break;
                }
            };
            let answer: f64 = calc_celsius(y);
            print!("{y:.2} Farenheit is equal to {answer:.2} Celsius");
            break;
        } else if x == 2 {
            let mut y = String::new();
            print!("Enter temperature in Celsius: ");
            io::stdout().flush().expect("Failed to flush!");
            rd_line(&mut y);
            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Given entry is not a number!");
                    break;
                }
            };
            let answer: f64 = calc_farenheit(y);
            print!("{y:.2} Celsius is equal to {answer:.2} Farenheit");
            break;
        } else {
            print!("Provide option 1 or 2 only!\nPlease choose your option: ");
            io::stdout().flush().expect("Failed to flush!");
            continue;
        }
    }
}
