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

trait AuthenticateUser {
    fn authenticate(&self) -> bool;
}

// Blanket implementation:
//
// All structures `T`, which implement `IdentifyUser`, will use this implementation of
// `AuthenticateUser`.
impl<T> AuthenticateUser for T
where
    T: IdentifyUser,
{
    fn authenticate(&self) -> bool {
        // Since every implementer of `IdentifyUser` has the `get_user_id` method, we can use it
        // within our authentication method.
        //
        // No additional edits need to be made to the `User` struct. Any new implementers of
        // `IdentifyUser` will also automatically be able to authenticate
        self.get_user_id() % 2 == 0
    }
}

fn main() {
    let user = User { user_id: 42 };

    // using the `get_user_id` method from the `IdentifyUser` trait
    println!("User ID: {}", user.get_user_id());

    // using the `authenticate` method from the `AuthenticateUser` supertrait
    println!("Authenticated: {}", user.authenticate());
}
