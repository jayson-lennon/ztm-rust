mod greet {
    pub fn hello() {
        println!("hello");
    }
    pub fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    use greet::hello;
    hello();
    greet::goodbye();
    math::add(1, 1);
}
