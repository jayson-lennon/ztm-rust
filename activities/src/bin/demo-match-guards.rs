#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::redundant_guards)]

mod ex1 {
    enum Status {
        Error(i32),
        Info,
        Warn,
    }

    pub fn run() {
        let status = Status::Error(5);
        match status {
            Status::Error(s @ 3) => println!("error three"),
            Status::Error(s @ 5..=6) => println!("error 5 or 6: {}", s),
            Status::Error(s @ 4..=10) => println!("error three through ten: {}", s),
            Status::Error(s @ 18 | s @ 19) => println!("error 18 or 19"),
            Status::Error(s) => println!("error code: {}", s),
            Status::Info => println!("info"),
            Status::Warn => println!("warn"),
        }
    }
}

mod ex2 {

    enum Species {
        Finch,
        Hawk,
        Parrot,
    }
    struct Bird {
        age: usize,
        species: Species,
    }

    #[rustfmt::skip]
    pub fn run() {
        let hawk = Bird {
            age: 13,
            species: Species::Hawk,
        };

        match hawk {
            Bird { age: 4, .. } => println!("4 year old bird"),
            Bird { age: 4..=10 | 15..=20, .. } => println!("4-10 or 15-20 year old bird"),
            Bird { species: Species::Finch, .. } => println!("finch!"),
            Bird { .. } => println!("other bird"),
        }
    }
}

mod ex3 {

    #[derive(PartialEq, PartialOrd, Eq, Ord)]
    enum Difficulty {
        Easy,
        Normal,
        Hard,
    }

    pub fn run() {
        let stage = 5;
        let diff = Difficulty::Normal;
        match stage {
            s if (s == 5 && diff == Difficulty::Easy) => println!("easy mode stage 5"),
            s if diff == Difficulty::Normal => println!("normal difficulty stage {}", s),
            s @ 10 | s @ 15 => println!("stage 10 or 15"),
            s => println!("stage {}", stage),
        }
    }
}

mod ex4 {

    struct Vehicle {
        km: usize,
        year: usize,
    }

    pub fn run() {
        let car = Vehicle {
            km: 80_000,
            year: 2020,
        };
        match car {
            Vehicle { km, year } if km == 0 && year == 2020 => println!("new 2020 vehicle"),
            Vehicle { km, .. } if km <= 50_000 => println!("under 50k km"),
            Vehicle { km, .. } if km >= 80_000 => println!("at least 80k km"),
            Vehicle { year, .. } if year == 2020 => println!("made in 2020"),
            Vehicle { .. } => println!("other mileage"),
        }
    }
}

fn main() {
    ex1::run();
    ex2::run();
    ex3::run();
    ex4::run();
}
