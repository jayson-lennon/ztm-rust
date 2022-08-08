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

pub fn capture_amount() -> Result<i32, ()> {
    loop {
        let input = capture_input("Enter amount:").unwrap_or("".to_owned());

        match input.parse::<i32>() {
            Ok(amount) => return Ok(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}
