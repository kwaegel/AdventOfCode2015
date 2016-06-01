
use std::{cmp, fmt};
use std::io;
use std::io::prelude::*;

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
enum Character {
    Player,
    Boss,
}

#[derive(Copy,Clone)]
struct Stats {
    health: i32,
    attack: i32,
    armor: i32,
    mana: i32,
}

#[derive(Clone,PartialEq,Eq,Debug)]
struct Spell {
    name: String,
    cost: i32, // Cost to cast
    duration: i32, // 0 means instant
    damage: i32, // Damage done to enemy
    healing: i32, // Healing factor for yourself
    armor: i32, // Bonus armor amount
    mana: i32, // Recharge amount
}

impl Spell {
    fn new(name: &str,
           cost: i32,
           duration: i32,
           damage: i32,
           healing: i32,
           armor: i32,
           mana: i32)
           -> Spell {
        Spell {
            name: String::from(name),
            cost: cost,
            duration: duration,
            damage: damage,
            healing: healing,
            armor: armor,
            mana: mana,
        }
    }
}
impl Spell {
    fn print_initial_cast_message(&self) {
        print!("Player casts {}", self);
        if self.armor > 0 {
            print!(", increasing armor by {}", self.armor);
        }
        if self.damage > 0 && self.duration == 0 {
            print!(", dealing {} damage", self.damage);
        }
        if self.healing > 0 && self.duration == 0 {
            print!(", and healing {} hit points", self.healing);
        }
        println!(".");
    }
}
impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
struct Effect {
    spell: Spell,
    turns_remaining: i32,
}

#[derive(Clone)]
struct BattleState {
    player: Stats,
    boss: Stats,
    effects: Vec<Effect>,
    mana_used: i32,
    turn: i32,
}
impl BattleState {
    fn print_state(&self, active_character: Character, verbose: bool) {
        if verbose {
            println!("");
        };
        if verbose {
            println!("--{:?} turn {}--", active_character, self.turn);
        };

        // Print stats
        if verbose {
            println!("Player has {} hit points, {} armor, {} mana",
                     self.player.health,
                     self.player.armor,
                     self.player.mana);
        };

        if verbose {
            println!("{} active spells", self.effects.len());
        }

        if verbose {
            println!("Boss has {} hit points", self.boss.health);
        };
    }

    fn apply_ongoing_effects(&mut self, verbose: bool) {
        for effect in self.effects.iter_mut() {
            effect.turns_remaining -= 1;

            if effect.spell.armor > 0 {
                if verbose {
                    println!("Shield's timer is now {}.", effect.turns_remaining);
                };
                if effect.turns_remaining > 0 {
                    self.player.armor = effect.spell.armor;
                } else {
                    if verbose {
                        println!("Shield wears off, decreasing armor by 7.");
                    };
                    self.player.armor -= effect.spell.armor;
                }
            }
            if effect.spell.damage > 0 {
                self.boss.health -= effect.spell.damage;
                if verbose {
                    println!("Poison deals 3 damage; its timer is now {}.",
                             effect.turns_remaining);
                };
                if verbose {
                    println!("Boss health now {}.", self.boss.health);
                };
            }
            if effect.spell.mana > 0 {
                self.player.mana += effect.spell.mana;
                if verbose {
                    println!("Recharge provides 101 mana (now {}); its timer is now {}",
                             self.player.mana,
                             effect.turns_remaining);
                };
            }
        }
        // Remove timed-out effects
        self.effects = self.effects.iter().filter(|s| s.turns_remaining > 0).cloned().collect();
    }

    fn is_spell_active(&self, spell: &Spell) -> bool {
        self.effects.iter().any(|ref s| s.spell.name == spell.name)
    }
}

fn pause() {
    // Read a single byte and discard
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}


fn print_player_turn(initial_state: &BattleState, spell_to_cast: Option<&Spell>) {
    // Replay all state messages and effects on a copy of the battle.
    let mut state = initial_state.clone();

    state.print_state(Character::Player, true);
    state.apply_ongoing_effects(true);

    match spell_to_cast {
        Some(ref spell) => spell.print_initial_cast_message(),
        None => (),
    }

    if state.player.mana < 53 {
        println!("##### Insufficent mana of {}. Player died #####",
                 state.player.mana);
    }
    if state.player.health <= 0 {
        println!("##### Player died #####");
    }
    if state.boss.health <= 0 {
        println!("***** Boss died *****");
    }
    pause();
}

fn print_boss_turn(initial_state: BattleState, damage: i32) {
    // Replay all state messages and effects on a copy of the battle.
    let mut state = initial_state.clone();

    state.print_state(Character::Boss, true);
    state.apply_ongoing_effects(true);

    if state.boss.health <= 0 {
        println!("***** Boss died *****");
        return;
    }

    println!("Boss attacks for {} damage.", damage);
    state.player.health -= damage;
    if state.player.health <= 0 {
        println!("##### Player died #####");
    }
}



// Run a round of battle and return the lowest amount of mana used to win.
// i32::max_value indicates a loss.
fn take_turn(initial_state: BattleState,
             active_character: Character,
             possible_spells: &Vec<Spell>)
             -> i32 {

    let verbose = false;
    //let verbose = true;

    let mut state = initial_state.clone();
    state.apply_ongoing_effects(false);

    // Check for victory from spell effects
    if state.boss.health <= 0 {
        // Player wins
        if verbose {
            print_player_turn(&initial_state, None);
        }
        return 0;
    }

    // if Player is active, cast spell
    if active_character == Character::Player {

        if state.player.mana < 53 {
            if verbose {
                print_player_turn(&initial_state, None);
            }
            return i32::max_value();
        }

        // Player casts spell
        let mut current_lowest_cost = i32::max_value();
        for spell_to_cast in possible_spells {

            let mut next_state = state.clone();

            if spell_to_cast.cost <= next_state.player.mana &&
               !next_state.is_spell_active(&spell_to_cast) {

                // Check for victory from spell effects
                if next_state.boss.health <= 0 {
                    // Player wins
                    // Didn't need to cast any spells, so zero cost. No need to continue checking.
                    if verbose {
                        print_player_turn(&initial_state, Some(spell_to_cast));
                    }
                    return 0;
                }

                // Subtract casting cost
                next_state.player.mana -= spell_to_cast.cost;

                if spell_to_cast.duration > 0 {
                    // Store delayed effects
                    next_state.effects.push(Effect {
                        spell: spell_to_cast.clone(),
                        turns_remaining: spell_to_cast.duration,
                    });
                } else {
                    // Apply immediate effects
                    next_state.boss.health -= spell_to_cast.damage;
                    next_state.player.health += spell_to_cast.healing;
                }

                // If boss is still alive, continue battle
                if next_state.boss.health > 0 {
                    next_state.turn += 1;

                    if verbose {
                        print_player_turn(&initial_state, Some(spell_to_cast));
                    }
                    let recursive_result = take_turn(next_state, Character::Boss, &possible_spells);
                    let total_cost = i32::saturating_add(recursive_result, spell_to_cast.cost);

                    current_lowest_cost = cmp::min(total_cost, current_lowest_cost);
                } else {
                    // Player wins
                    if verbose {
                        print_player_turn(&initial_state, Some(spell_to_cast));
                        println!("***** Boss died *****");
                    }
                    current_lowest_cost = cmp::min(spell_to_cast.cost, current_lowest_cost);
                }
            }
        }
        return current_lowest_cost;
    } else {
        // Boss's turn
        let mut next_state = state.clone();

        // Calculate damage
        let damage = next_state.boss.attack - next_state.player.armor;
        let damage = cmp::max(1, damage);
        next_state.player.health -= damage;

        if verbose {
            print_boss_turn(initial_state, damage);
        }

        if next_state.player.health > 0 {
            // Player is still alive to take a turn
            next_state.turn += 1;
            let cost = take_turn(next_state, Character::Player, &possible_spells);
            return cost;
        } else {
            return i32::max_value();    // Player died
        }
    };
}



fn main() {


    // Magic Missile costs 53 mana. It instantly does 4 damage.
    // Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
    // Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
    // Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
    // Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.

    // name, cost, duration, damage, healing, armor, mana
    let possible_spells = vec![
        Spell::new("Magic Missle", 53, 0, 4, 0, 0, 0),
        Spell::new("Drain",        73, 0, 2, 2, 0, 0),
        Spell::new("Shield",      113, 6, 0, 0, 7, 0),
        Spell::new("Poison",      173, 6, 3, 0, 0, 0),
        Spell::new("Recharge",    229, 5, 0, 0, 0, 101),
    ];



    // Hit Points: 71
    // Damage: 10
    let starting_boss = Stats {health: 71, attack: 10, armor: 0, mana: 0};

    // You start with 50 hit points and 500 mana points.
    let starting_player = Stats {health: 50, attack: 0, armor: 0, mana: 500};

    // // Test characters
    // let starting_boss = Stats {
        // health: 14,
        // attack: 8,
        // armor: 0,
        // mana: 0,
    // };
    // let starting_player = Stats {
        // health: 10,
        // attack: 0,
        // armor: 0,
        // mana: 250,
    // };

    
    let initial_initial_state = BattleState {
        player: starting_player,
        boss: starting_boss,
        effects: Vec::new(),
        mana_used: 0,
        turn: 0,
    };

    let mana_used = take_turn(initial_initial_state, Character::Player, &possible_spells);

    println!("\n######");
    println!("Final mana used: {:?}", mana_used);

    assert!(mana_used == 1824);
}
