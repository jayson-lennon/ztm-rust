pub fn divider(length: Option<i8>, char: Option<String>) {
    println!(
        "{:?}",
        char.unwrap_or("=".to_owned())
            .repeat((length.unwrap_or(5) as usize))
    );
}
