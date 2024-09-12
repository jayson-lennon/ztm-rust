fn main() {
    let my_name = "Bob";
    match my_name {
        "Jayson" => println!("that is my name"),
        "Bob" => println!("that is my name"),
        "Alice" => println!("hello alice"),
        _ => println!("nice to meet you!"),
    }
}
