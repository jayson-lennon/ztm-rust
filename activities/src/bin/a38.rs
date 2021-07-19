// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    use std::thread;

    let msg_one = thread::spawn(move || msg_hello());
    let msg_two = thread::spawn(move || msg_thread());
    let msg_three = thread::spawn(move || msg_excited());

    let msg_one = msg_one.join().expect("failed to join msg one");
    let msg_two = msg_two.join().expect("failed to join msg two");
    let msg_three = msg_three.join().expect("failed to join msg three");

    println!("{}{}{}", msg_one, msg_two, msg_three);
}
