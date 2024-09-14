#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::vec_init_then_push)]

macro_rules! myvec {
    (
        $($element:expr),+
        $(,)?
    ) => {{
        let mut v = Vec::new();

        $(
            v.push($element);
        )+

        v
    }}
}

fn main() {
    let v = myvec!(1, 2, 3, 4);
    let v = {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v
    };
}
