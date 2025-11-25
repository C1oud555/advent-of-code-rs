use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

#[derive(Clone, Debug, Copy)]
struct Context {
    player_hit_point: isize,
    player_avai_mana: isize,
    player_avai_armor: isize,
    boss_hit_point: isize,
    shield_timer: isize,
    poison_timer: isize,
    recharge_timer: isize,
}

const BOSS_HITPOINT: isize = 58;
const BOSS_DMAGE: isize = 9;

#[derive(Debug, Clone, Copy)]
struct Spell {
    cost: isize,
    damage: isize,
    heal: isize,
    timer: isize,
}

const SPELLS: [Spell; 5] = [
    Spell {
        cost: 53,
        damage: 4,
        heal: 0,
        timer: 0,
    }, // Magic Missile
    Spell {
        cost: 73,
        damage: 2,
        heal: 2,
        timer: 0,
    }, // Drain
    Spell {
        cost: 113,
        damage: 0,
        heal: 0,
        timer: 6,
    }, // Shield
    Spell {
        cost: 173,
        damage: 3,
        heal: 0,
        timer: 6,
    }, // Poison
    Spell {
        cost: 229,
        damage: 0,
        heal: 0,
        timer: 5,
    }, // Recharge
];

fn solve(state: Context, mana_spent: isize, min_mana: &mut isize, player_turn: bool, part2: bool) {
    // Pruning
    if mana_spent >= *min_mana {
        return;
    }

    let mut current_state = state;

    // Part 2 Hard Mode
    if part2 && player_turn {
        current_state.player_hit_point -= 1;
        if current_state.player_hit_point <= 0 {
            return;
        }
    }

    // Apply effects at start of turn
    current_state.player_avai_armor = 0;
    if current_state.shield_timer > 0 {
        current_state.player_avai_armor = 7;
        current_state.shield_timer -= 1;
    }
    if current_state.poison_timer > 0 {
        current_state.boss_hit_point -= 3;
        current_state.poison_timer -= 1;
    }
    if current_state.recharge_timer > 0 {
        current_state.player_avai_mana += 101;
        current_state.recharge_timer -= 1;
    }

    // Check for win condition after effects
    if current_state.boss_hit_point <= 0 {
        if mana_spent < *min_mana {
            *min_mana = mana_spent;
        }
        return;
    }

    if player_turn {
        // Player's turn: try every possible spell
        for (i, spell) in SPELLS.iter().enumerate() {
            // Check if we can cast the spell
            if current_state.player_avai_mana < spell.cost {
                continue;
            }
            if i == 2 && current_state.shield_timer > 0 {
                continue;
            }
            if i == 3 && current_state.poison_timer > 0 {
                continue;
            }
            if i == 4 && current_state.recharge_timer > 0 {
                continue;
            }

            let mut next_state = current_state;
            next_state.player_avai_mana -= spell.cost;
            let new_mana_spent = mana_spent + spell.cost;

            if spell.timer == 0 {
                // Instant spells
                next_state.boss_hit_point -= spell.damage;
                next_state.player_hit_point += spell.heal;
            } else {
                // Effect spells
                match i {
                    2 => next_state.shield_timer = spell.timer,
                    3 => next_state.poison_timer = spell.timer,
                    4 => next_state.recharge_timer = spell.timer,
                    _ => unreachable!(),
                }
            }

            // Check for win right after casting
            if next_state.boss_hit_point <= 0 {
                if new_mana_spent < *min_mana {
                    *min_mana = new_mana_spent;
                }
                continue; // Don't proceed to boss turn if already won
            }

            solve(next_state, new_mana_spent, min_mana, false, part2);
        }
    } else {
        // Boss's turn
        let mut next_state = current_state;
        let damage = (BOSS_DMAGE - next_state.player_avai_armor).max(1);
        next_state.player_hit_point -= damage;

        if next_state.player_hit_point > 0 {
            solve(next_state, mana_spent, min_mana, true, part2);
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let context = Context {
        player_hit_point: 50,
        player_avai_mana: 500,
        player_avai_armor: 0,
        boss_hit_point: BOSS_HITPOINT,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
    };
    let mut min_mana = isize::MAX;
    solve(context, 0, &mut min_mana, true, false);
    format_result!(min_mana)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let context = Context {
        player_hit_point: 50,
        player_avai_mana: 500,
        player_avai_armor: 0,
        boss_hit_point: BOSS_HITPOINT,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
    };
    let mut min_mana = isize::MAX;
    solve(context, 0, &mut min_mana, true, true);
    format_result!(min_mana)
}
