use std::io;

pub fn get_str(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn get_int(prompt: &str) -> i32 {
    let mut input = get_str(prompt);
    while input.parse::<i32>().is_err() {
        println!("Invalid input. Please try again.");
        input = get_str(prompt);
    }

    input.parse::<i32>().unwrap()
}
