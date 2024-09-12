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

impl IntoIterator for FruitStand {
    type Item = (Fruit, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.into_iter()
    }
}

impl<'a> IntoIterator for &'a FruitStand {
    type Item = (&'a Fruit, &'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter()
    }
}

impl<'a> IntoIterator for &'a mut FruitStand {
    type Item = (&'a Fruit, &'a mut u32);
    type IntoIter = std::collections::hash_map::IterMut<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter_mut()
    }
}

fn main() {
    let mut fruit = HashMap::new();
    fruit.insert(Fruit::Banana, 5);
    fruit.insert(Fruit::Apple, 2);
    fruit.insert(Fruit::Orange, 6);

    let fruit = fruit;

    let mut store = FruitStand { fruit };

    for (fruit, stock) in &mut store {
        *stock += 10;
        println!("{:?}: {:?}", fruit, stock);
    }
}
