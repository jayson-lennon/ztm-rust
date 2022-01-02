macro_rules! html {
}

#[allow(unused_must_use)]
fn main() {
    use std::fmt::Write;
    let mut data = String::new();
    html!(&mut data,
    html[
        head[ title["Demo title"] ]
        body[
            h1["Sample"]
            p["This is a macro demo"]
        ]
    ]);
    dbg!(data);
}
