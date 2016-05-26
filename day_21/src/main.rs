
use std::{cmp, fmt};

#[derive(Copy,Clone)]
struct Stats {
    health: i32,
    attack: i32,
    defence: i32,
}

impl Stats {
    fn add_item(&mut self, item: &Item) {
        self.attack += item.damage;
        self.defence += item.armor;
    }
}

#[derive(PartialEq,Eq)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn new(name: &str, cost: i32, damage: i32, armor: i32 ) -> Item {
        Item{name: String::from(name), cost: cost, damage: damage, armor: armor}
    }
}

impl fmt::Display for Item {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.name)
    }
}

// Player 1 attacks first, and thus wins on ties.
// Returns 1 if player 1 wins, 2 if player 2 wins.
fn player_wins(player: Stats, boss: Stats, verbose: bool) -> bool {
    let player_hp_loss_per_hit = cmp::max(1, boss.attack - player.defence);
    let boss_hp_loss_per_hit = cmp::max(1, player.attack - boss.defence);

    // Solve the linear equation { hp = hp_loss*num_hits + 100 } for num_hits to find the winner.
    let boss_dead_after = 100.0/boss_hp_loss_per_hit as f32;
    let player_dead_after = 100.0/player_hp_loss_per_hit as f32;
    
    if verbose {
        println!("player hp loss {}, boss hp loss {}", player_hp_loss_per_hit, boss_hp_loss_per_hit);
        println!("boss dead after {} hits, player dead after {} hits", boss_dead_after, player_dead_after);
    }
    
    boss_dead_after <= player_dead_after
}

fn main() {
    println!("Hello, world!");
    
    let weapons = vec![
        Item::new("no weapon",        0, 0, 0),
        Item::new("Dagger",      8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer",  25, 6, 0),
        Item::new("Longsword",  40, 7, 0),
        Item::new("Greataxe",   74, 8, 0),
    ];
    
    let armors = vec![
        Item::new("no armor",         0, 0, 0),
        Item::new("Leather",     13, 0, 1),
        Item::new("Chainmail",   31, 0, 2),
        Item::new("Splintmail",  53, 0, 3),
        Item::new("Bandedmail",  75, 0, 4),
        Item::new("Platemail",  102, 0, 5),
    ];
    
    let rings = vec![
        Item::new("no ring",        0, 0, 0),
        Item::new("Damage +1",  25, 1, 0),
        Item::new("Damage +2",  50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defence +1", 20, 0, 1),
        Item::new("Defence +2", 50, 0, 2),
        Item::new("Defence +3", 80, 0, 3),
    ];
    
    
    // Hit Points: 100
    // Damage: 8
    // Armor: 2
    let boss = Stats {health: 100, attack: 8, defence: 2};
    
    let base_player = Stats {health: 100, attack: 0, defence: 0};
    
    let mut lowest_cost = i32::max_value();
    
    // There are few enough possible combinations for a brute-force search.
    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1 != ring2 {
                        let mut player = base_player;
                        player.add_item(&weapon);
                        player.add_item(&armor);
                        player.add_item(&ring1);
                        player.add_item(&ring2);
                        let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                        
                        let victory = player_wins(player, boss, false);
                        
                        if victory && cost < lowest_cost {
                            player_wins(player, boss, true);
                            println!("Victory! for {} gold with {}, {}, {}, {},", cost, weapon, armor, ring1, ring2);
                            println!("");
                            lowest_cost = cost;
                        }
                    }
                }
            }
        }
    }
    
    // Solution for part 1 is 91 gold.
    assert_eq!(lowest_cost, 91);
}
