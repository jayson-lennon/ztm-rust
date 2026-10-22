// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let my_vec: Vec<i32> = vec![10, 20, 30, 40];

    for i in &my_vec {
        if *i == 30 {
            println!("thirty")
        } else {
            println!("{i}")
        }
    }

    println!("==========");
    println!("Len is {}", my_vec.len());
}
