fn main() {
    let s: String = String::from("MakaPaka");
    let (s, len) = calculate_length(s);

    println!("The string is: {s}, the length is: {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let len: usize = s.len();
    (s, len)
}
