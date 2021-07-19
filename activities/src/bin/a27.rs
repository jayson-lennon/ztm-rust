// Topic: Custom error types
//
// Requirements:
// * Modify the `ProgramError` enum in order to make the program compile
//   and run. Do not modify any other part of the program.
// * The output should display a menu error followed by a math error when running.
//
// Notes:
// * Use `#[error("description")]` on the enum variants
// * Use `#[from] ErrorType` to convert the existing errors into a `ProgramError`

use thiserror::Error;

enum ProgramError {}

#[derive(Debug, Error)]
enum MenuError {
    #[error("menu item not found")]
    NotFound,
}

#[derive(Debug, Error)]
enum MathError {
    #[error("divide by zero error")]
    DivideByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError> {
    match choice {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => Err(MenuError::NotFound),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(MathError::DivideByZero)
    }
}

fn run(step: i32) -> Result<(), ProgramError> {
    if step == 1 {
        pick_menu("4")?;
    } else if step == 2 {
        divide(1, 0)?;
    }
    Ok(())
}

fn main() {
    println!("{:?}", run(1));
    println!("{:?}", run(2));
}
