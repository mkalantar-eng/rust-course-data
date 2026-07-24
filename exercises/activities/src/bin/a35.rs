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
// * Everything else should not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

use crate::Tile::{Grass, Treasure, Water, Wood};

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

fn print_tile(tile: &Tile) {
    use BrickStyle::*;
    use Tile::*;
    match tile {
        Brick(style @ Red | style @ Gray) => println!("The brick color is {:?}", style),
        Brick(style @ Dungeon) => println!("{:?} brick", style),
        Water(Pressure(_p @ 10..)) => println!("High water pressure!"),
        Water(Pressure(p @ ..10)) => println!("Water pressure level: {:?}", p),
        Grass | Dirt | Sand => println!("Ground tile"),
        Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: 100..,
        }) => println!("Lots of gold!"),
        Treasure(TreasureChest { .. }) | Wood => {}
    }
}

fn main() {
    print_tile(&Tile::Brick(BrickStyle::Gray));
    print_tile(&Tile::Brick(BrickStyle::Dungeon));
    print_tile(&Water(Pressure(20)));
    print_tile(&Water(Pressure(2)));
    print_tile(&Grass);
    print_tile(&Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 1000,
    }));
    print_tile(&Treasure(TreasureChest {
        content: TreasureItem::SuperPower,
        amount: 100,
    }));
    print_tile(&Wood);
}
