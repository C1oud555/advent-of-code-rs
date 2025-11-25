use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

#[derive(Clone, Debug)]
struct Player {
    hit_point: isize,
    damage: isize,
    armor: isize,
}

const BOSS: Player = Player {
    hit_point: 103,
    damage: 9,
    armor: 2,
};

const WEAPONS: [(isize, isize, isize); 5] =
    [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const ARMORS: [(isize, isize, isize); 5] =
    [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
const RINGS: [(isize, isize, isize); 6] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

fn get_all_setups() -> Vec<(isize, isize, isize)> {
    let mut setups = Vec::new();

    // Ring combinations
    let no_ring = std::iter::once((0, 0, 0));
    let one_ring = RINGS.iter().cloned();
    let two_rings = (0..RINGS.len()).flat_map(|i| {
        ((i + 1)..RINGS.len()).map(move |j| {
            (
                RINGS[i].0 + RINGS[j].0,
                RINGS[i].1 + RINGS[j].1,
                RINGS[i].2 + RINGS[j].2,
            )
        })
    });
    let all_ring_combos: Vec<_> = no_ring.chain(one_ring).chain(two_rings).collect();

    // Armor combinations (including no armor)
    let all_armor_combos: Vec<_> = ARMORS
        .iter()
        .cloned()
        .chain(std::iter::once((0, 0, 0)))
        .collect();

    for &weapon in &WEAPONS {
        for &armor in &all_armor_combos {
            for &ring_combo in &all_ring_combos {
                setups.push((
                    weapon.0 + armor.0 + ring_combo.0,
                    weapon.1 + armor.1 + ring_combo.1,
                    weapon.2 + armor.2 + ring_combo.2,
                ));
            }
        }
    }
    setups
}

fn does_player_win(player_damage: isize, player_armor: isize) -> bool {
    let player_hp = 100;
    let boss_hp = BOSS.hit_point;
    let boss_damage = BOSS.damage;
    let boss_armor = BOSS.armor;

    let player_attack = (player_damage - boss_armor).max(1);
    let boss_attack = (boss_damage - player_armor).max(1);

    let turns_to_win = (boss_hp + player_attack - 1) / player_attack;
    let turns_to_lose = (player_hp + boss_attack - 1) / boss_attack;

    turns_to_win <= turns_to_lose
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let min_cost_to_win = get_all_setups()
        .into_iter()
        .filter(|&(_, damage, armor)| does_player_win(damage, armor))
        .map(|(cost, _, _)| cost)
        .min()
        .unwrap_or(0);

    format_result!(min_cost_to_win)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let max_cost_to_lose = get_all_setups()
        .into_iter()
        .filter(|&(_, damage, armor)| !does_player_win(damage, armor))
        .map(|(cost, _, _)| cost)
        .max()
        .unwrap_or(0);

    format_result!(max_cost_to_lose)
}
