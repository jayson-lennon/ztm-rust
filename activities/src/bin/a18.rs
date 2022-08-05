// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

impl Customer {
    fn try_purchase(&self) -> Result<(), String> {
        let is_overage = self.age > 18;

        match is_overage {
            true => Ok(()),
            false => Err("Customer must be at least 18 years old".to_owned()),
        }
    }
}

fn main() {
    let customers = vec![Customer { age: 17 }, Customer { age: 19 }];

    for customer in customers {
        println!("{:?}", customer.try_purchase());
    }
}
