struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {name}");
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}
