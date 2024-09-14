#![allow(dead_code)]
#![allow(clippy::match_like_matches_macro)]

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("{:?}", can_access_file);
}
