// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn calc() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
        .iter()
        .map(|n| n * 3)
        .filter(|n| n > &10)
        .collect()
}

fn main() {
    let data = calc();

    for n in data {
        println!("{}", n);
    }
}

#[cfg(test)]
mod test {
    use crate::calc;

    #[test]
    fn test_calc() {
        assert_eq!(calc(), vec![12, 15])
    }
}
