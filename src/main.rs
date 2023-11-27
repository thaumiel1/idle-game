// Game data storage

use std::{cmp, time};

struct GameData {
    building_index: BuildingIndex,
    item_index: ItemIndex,
}

struct BuildingIndex {
    iron_mine: usize,
    iron_forge: usize,
}

struct ItemIndex {
    iron_ore: usize,
    iron_plate: usize,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            building_index: Default::default(),
            item_index: Default::default(),
        }
    }
}

impl Default for BuildingIndex {
    fn default() -> Self {
        BuildingIndex {
            iron_mine: 0,
            iron_forge: 0,
        }
    }
}

impl Default for ItemIndex {
    fn default() -> Self {
        ItemIndex {
            iron_ore: 0,
            iron_plate: 0,
        }
    }
}

// Game logic
/*
Aiming for 5 ticks a second.
*/

fn tick(game: &mut GameData) {
    if game.building_index.iron_mine != 0 {
        game.item_index.iron_ore += (1 * game.building_index.iron_mine);
    }

    if game.building_index.iron_forge != 0 && game.item_index.iron_ore != 0 {
        game.item_index.iron_plate += cmp::min(game.item_index.iron_ore, game.building_index.iron_forge);
    }
}

fn main() {
    println!("Hello, world!");
    let mut game : GameData = Default::default();
    let mut count = 0;
    loop {
        std::thread::sleep(time::Duration::from_millis(200));
        tick(&mut game);
        println!("{}", format!("Iron ore: {}, Iron plates: {}", game.item_index.iron_ore, game.item_index.iron_plate));
        count += 1;
        println!("Ticks: {}", count)
    }
}
