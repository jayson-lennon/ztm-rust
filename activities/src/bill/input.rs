use std::io;

pub fn capture_input(message: &str) -> Option<String> {
    let mut buffer = String::new();
    println!("{:?}", message);

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Try again")
    }

    let input = buffer.trim();

    match input {
        "" => None,
        _ => Some(input.to_owned()),
    }
}
