// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.

macro_rules! hashmap {
    (
        $($key:expr => $value:expr),+
        $(,)?
    ) => {{
        use ::std::collections::HashMap;
        let mut hashmap = HashMap::new();
        $(
            hashmap.insert($key, $value);
        )+
        hashmap
    }};
}

fn main() {
    let hashmap = hashmap!(
        1: "a".to_owned(),
        2: "b".to_owned(),
        3: "c".to_owned(),
    );
    dbg!(hashmap);
}
