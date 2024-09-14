#![allow(unused_variables)]

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();

    let plus_one_with_filter: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .filter(|num| num < &1)
        .collect();

    let new_numbers: Vec<_> = numbers.iter().filter(|num| num <= &&3).collect();

    let numbers = [1, 2, 3, 4, 5];
    let find_me: Option<&i32> = numbers.iter().find(|num| num == &&40);

    let count = numbers.iter().count();

    let last: Option<&i32> = numbers.iter().last();

    let numbers = [1, 2, 3, 4, 5];
    let min: Option<&i32> = numbers.iter().min();
    let max: Option<&i32> = numbers.iter().max();

    let take: Vec<&i32> = numbers.iter().take(3).collect();
}
