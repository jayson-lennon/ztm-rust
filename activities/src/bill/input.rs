use std::io;

pub fn capture_input(message: &str) -> String {
    let mut buffer = String::new();
    println!("{:?}", message);

    let mut input = io::stdin().read_line(&mut buffer);

    while input.is_err() {
        input = io::stdin().read_line(&mut buffer);
    }

    return buffer.trim().to_owned();
}
