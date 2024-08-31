trait IdentifyUser {
    // return user id to identify them
    fn get_user_id(&self) -> u32;
}

struct User {
    user_id: u32,
}

impl IdentifyUser for User {
    fn get_user_id(&self) -> u32 {
        self.user_id
    }
}

fn main() {
    let user = User { user_id: 42 };

    // using the `get_user_id` method from the `IdentifyUser` trait
    println!("User ID: {}", user.get_user_id());
}
