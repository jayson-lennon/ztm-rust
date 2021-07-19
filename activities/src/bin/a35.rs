// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    use Tile::*;
    match tile {
        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {:?}", brick)
        }
        Brick(other) => println!("{:?} brick", other),
        Dirt | Grass | Sand => println!("Ground tile"),
        Treasure(TreasureChest { amount, .. }) if amount >= 100 => println!("Lots of gold!"),
        Water(pressure) if pressure.0 < 10 => println!("Water pressure level: {}", pressure.0),
        Water(pressure) if pressure.0 >= 10 => println!("High water pressure!"),
        _ => (),
    }
}

fn main() {
    let tile = Tile::Brick(BrickStyle::Red);
    print_tile(tile);

    let tile = Tile::Sand;
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 200,
    });
    print_tile(tile);

    let tile = Tile::Water(Pressure(9));
    print_tile(tile);
}
