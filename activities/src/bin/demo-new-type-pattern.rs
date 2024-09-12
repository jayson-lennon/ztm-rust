#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    pub fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("cannot be zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    a / b
}

fn main() {
    match NeverZero::new(0) {
        Ok(nz) => println!("{:?}", divide(10, nz)),
        Err(e) => println!("{e:?}"),
    }
}
