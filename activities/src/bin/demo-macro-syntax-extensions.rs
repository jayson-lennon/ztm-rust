#[allow(unused_macros)]
macro_rules! iterslice {
    // iterable[start:end-1]
    // iterable[1:3] -> items at index 1 and 2
    ($iterable:ident [ $start:literal : $end:literal ] ) => {{
        let iterable = $iterable.iter();
        if $start > $end {
            panic!("start index is > end index");
        }
        if $start == $end {
            iterable.skip($start).take(1)
        } else {
            // Don't need to check if end > start. Compiler will check for us.
            iterable.skip($start).take($end - $start)
        }
    }};

    // iterable[k]
    // iterable[3] -> retrieve item at index 3
    ($iterable:ident [ $index:literal ] ) => {{
        let mut iterable = $iterable.iter();
        iterable.nth($index).expect("index out of bounds")
    }};

    // iterable[start:]
    // iterable[3:] -> skip 3, until the end
    ($iterable:ident [ $index:literal : ] ) => {{
        let iterable = $iterable.iter();
        iterable.skip($index)
    }};

    // iterable[:end-1]
    // iterable[:5] -> take everything up to (but not including) index 5
    ($iterable:ident [ : $index:literal] ) => {{
        let iterable = $iterable.iter();
        iterable.take($index)
    }};
}

fn main() {
    // iterable[start:end-1]
    // iterable[1:3] -> items at index 1 and 2
    {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let new_iter = {
            let iterable = numbers.iter();
            let (start, end) = (1, 5);
            if start == end {
                iterable.skip(start).take(1)
            } else {
                iterable.skip(start).take(end - start)
            }
        };

        for i in new_iter {
            dbg!(i);
        }

        let new_slice = iterslice!(numbers[1:5]);
        for i in new_slice {
            dbg!(i);
        }
    }

    // iterable[k]
    {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let value = {
            let mut iterable = numbers.iter();
            let index = 3;
            iterable.nth(index).expect("index out of bounds")
        };

        dbg!(value);

        let v = iterslice!(numbers[4]);
        dbg!(v);
    }

    // iterable[start:]
    // iterable[3:] -> skip 3, until the end
    {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let new_iter = {
            let start = 3;
            let iterable = numbers.iter();
            iterable.skip(start)
        };
        println!("start:");
        for i in new_iter {
            dbg!(i);
        }

        let test = iterslice!(numbers[3:]);

        println!("--test--");
        for i in test {
            dbg!(i);
        }
    }

    // iterable[:end-1]
    {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let new_iter = {
            let end = 4;
            let iterable = numbers.iter();
            iterable.take(4)
        };

        println!(":end");
        for i in new_iter {
            dbg!(i);
        }
        let test = iterslice!(numbers[:4]);
        println!("--test--");
        for i in test {
            dbg!(i);
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn iter_range() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let new_iter = iterslice!(numbers[1:5]);
        let expected = vec![2, 3, 4, 5];

        for (expect, actual) in new_iter.zip(expected.iter()) {
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn iter_until_n() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let new_iter = iterslice!(numbers[:4]);
        let expected = vec![1, 2, 3, 4];

        for (expect, actual) in new_iter.zip(expected.iter()) {
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn iter_from_n_until_end() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let new_iter = iterslice!(numbers[2:]);
        let expected = vec![3, 4, 5, 6];

        for (expect, actual) in new_iter.zip(expected.iter()) {
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn get_index() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let value = iterslice!(numbers[4]);
        let expected = &5;
        assert_eq!(expected, value);
    }
}
