// Topic: External Modules
//
// Summary:
// The existing program is complete, but all the code exists
// in a single module. This code can benefit from being organized
// into multiple external modules.
//
// Requirements:
// * Organize the code into two external modules based on their functionality:
//   - msg: string formatting functions
//   - math: math functions
// * Update the main function to use the functionality from the modules
//
// Notes:
// * Update your Cargo.toml to include a library file
// * After moving the functions into modules, try running
//   `cargo check --bin a26c` to get a listing of required code changes

use mylib;

fn main() {
    use mylib::math;
    // Part 1: math functions
    let result = {
        let two_plus_two = math::add(2, 2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three)
    };

    // Ensure we have a correct result.
    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    {
        use mylib::msg::{capitalize, exciting, trim};
        // Part 2: string functions
        let hello = {
            let msg = "hello ";
            let msg = trim(msg);
            capitalize(msg)
        };
        let world = {
            let msg = "world";
            exciting(msg)
        };
        let msg = format!("{}, {}", hello, world);

        // Ensure we have a correct result.
        assert_eq!(&msg, "Hello, world!");
        println!("{}", msg);
    }
}
