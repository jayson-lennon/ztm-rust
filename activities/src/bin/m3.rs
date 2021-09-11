// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.

macro_rules! hashmap {
    (
        $($key:tt : $value:tt),+
        $(,)?
    ) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )+
        map
    }};
}

fn main() {
    let hashmap = hashmap!(
        "aaa": "111",
        "bbb": "222",
        "ccc": "333",
    );
    dbg!(hashmap);
}
