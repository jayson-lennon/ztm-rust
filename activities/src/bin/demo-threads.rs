mod ex1 {
    use std::thread;

    pub fn run() {
        let iterations = 10;

        let a = thread::spawn(move || {
            for i in 1..=iterations {
                println!("A:{}", i);
            }
        });

        let b = thread::spawn(move || {
            for i in 1..=iterations {
                println!("   B:{}", i);
            }
        });

        a.join();
        b.join();
    }
}

mod ex2 {

    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    pub fn run() {
        let value: JoinHandle<usize> = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            42
        });

        println!("Waiting for value...");

        match value.join() {
            Ok(n) => println!("value: {n}"),
            Err(e) => println!("error joining thread: {e:?}"),
        }
    }
}

mod ex3 {

    use std::thread;

    pub fn run() {
        let data = vec!['a', 'b', 'c'];
        let capitalized = thread::spawn(move || {
            let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
            data
        });

        println!("Waiting for value...");

        match capitalized.join() {
            Ok(caps) => println!("value: {caps:?}"),
            Err(e) => println!("error: {e:?}"),
        }
    }
}

fn main() {
    ex1::run();
    ex2::run();
    ex3::run();
}
