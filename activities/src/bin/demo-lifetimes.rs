#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City {
    Barland,
    Bazopolis,
    Fooville,
}

#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self {
            name: name.to_string(),
            age,
            city,
        }
    }
}

fn new_ids() -> Cards {
    Cards {
        inner: vec![
            IdCard::new("Amy", 1, City::Fooville),
            IdCard::new("Matt", 10, City::Barland),
            IdCard::new("Bailee", 20, City::Barland),
            IdCard::new("Anthony", 30, City::Bazopolis),
            IdCard::new("Tina", 40, City::Bazopolis),
        ],
    }
}

fn main() {
    let ids = new_ids();
}
