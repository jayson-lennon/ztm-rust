// Topic: Basic syntax extension macro
//
// Summary:
//   Create a syntax extension macro that allows selecting items out of an iterator
//   using human-readable terms.
//
// Requirements:
// * Implement the remaining macro_rules using the formats shown in the main function.
// * The type returned by the macro must match the annotations in the main function.
//
// Notes:
// * One matcher for the macro is provided & can be used as a guide.
// * Run `cargo test --bin m4` to check your work.

macro_rules! get {
    // first item from iterable
    (first item from $iterable:expr) => {
        $iterable.iter().nth(0)
    };
    // last item from iterable
    (last item from $iterable:expr) => {{
        let len = $iterable.len();
        $iterable.iter().skip(len - 1).nth(0)
    }};
    // first k items from iterable
    (first $count:literal items from $iterable:expr) => {
        $iterable.iter().take($count)
    };
    // last k items from iterable
    (last $count:literal items from $iterable:expr) => {{
        let len = $iterable.len();
        $iterable.iter().skip(len - $count)
    }};
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let first_3: Vec<&i32> = get!(first 3 items from data).collect::<Vec<_>>();
    let last_3: Vec<&i32> = get!(last 3 items from data).collect::<Vec<_>>();
    let first_item: Option<&i32> = get!(first item from data);
    let last_item: Option<&i32> = get!(last item from data);
}

#[cfg(test)]
mod test {
    #[test]
    fn first_item() {
        let data = vec![1, 2, 3, 4, 5];
        let first = get!(first item from data);
        assert_eq!(first.unwrap(), &1);
    }

    #[test]
    fn last_item() {
        let data = vec![1, 2, 3, 4, 5];
        let last = get!(last item from data);
        assert_eq!(last.unwrap(), &5);
    }

    #[test]
    fn first_k_items() {
        let data = vec![1, 2, 3, 4, 5];
        let first: Vec<_> = get!(first 3 items from data).collect();
        assert_eq!(first, vec![&1, &2, &3]);
    }

    #[test]
    fn last_k_items() {
        let data = vec![1, 2, 3, 4, 5];
        let last: Vec<_> = get!(last 3 items from data).collect();
        assert_eq!(last, vec![&3, &4, &5]);
    }
}
