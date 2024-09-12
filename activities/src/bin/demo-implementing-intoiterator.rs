use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Fruit {
    Apple,
    Banana,
    Orange,
}

struct FruitStand {
    fruit: HashMap<Fruit, u32>,
}

fn main() {
    let mut fruit = HashMap::new();
    fruit.insert(Fruit::Banana, 5);
    fruit.insert(Fruit::Apple, 2);
    fruit.insert(Fruit::Orange, 6);

    let fruit = fruit;

    let mut store = FruitStand { fruit };
}
