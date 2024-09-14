#![allow(unused_variables)]
#![allow(clippy::unnecessary_literal_unwrap)]

fn main() {
    let a: Option<i32> = Some(1);
    let a_is_some = a.is_some();
    let a_is_none = a.is_none();
    let a_mapped = a.map(|num| num + 1);
    let a_filtered = a.filter(|num| num == &1);
    let a_or_else = a.or(Some(5));
    let unwrapped = a.unwrap_or(0);
}
