use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

#[derive(Clone, Debug)]
struct Player {
    hit_point: isize,
    damage: isize,
    armor: isize,
    cost: isize,
}

const BOSS: Player = Player {
    hit_point: 103,
    damage: 9,
    armor: 2,
    cost: 0,
};

impl Player {
    fn add_equipment(&mut self, equip: (isize, isize, isize)) {
        self.cost += equip.0;
        self.damage += equip.1;
        self.armor += equip.2;
    }
}

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

fn get_all_players() -> Vec<Player> {
    let init = Player {
        hit_point: 100,
        damage: 0,
        armor: 0,
        cost: 0,
    };
    let mut ret = Vec::new();
    let weapon_armor_intersect = |player: &Player| -> Vec<Player> {
        let mut tmp = Vec::new();
        for weapon in WEAPONS {
            let mut tmp_player = player.clone();
            tmp_player.add_equipment(weapon);
            tmp.push(tmp_player);
        }
        for weapon in WEAPONS {
            for armor in ARMORS {
                let mut tmp_player = player.clone();
                tmp_player.add_equipment(weapon);
                tmp_player.add_equipment(armor);
                tmp.push(tmp_player);
            }
        }
        tmp
    };
    // 0 rings
    let inittmp = init.clone();
    ret.extend(weapon_armor_intersect(&inittmp));
    // 1 rings
    for ring in RINGS {
        let mut inittmp = init.clone();
        inittmp.add_equipment(ring);
        ret.extend(weapon_armor_intersect(&inittmp));
    }
    // 2 rings
    for ring0 in RINGS {
        for ring1 in RINGS {
            if ring0 != ring1 {
                let mut inittmp = init.clone();
                inittmp.add_equipment(ring0);
                inittmp.add_equipment(ring1);
                ret.extend(weapon_armor_intersect(&inittmp));
            }
        }
    }
    ret
}

fn fight(player: &Player) -> Option<isize> {
    let mut player = player.clone();
    let mut boss = BOSS.clone();
    loop {
        boss.hit_point -= (player.damage - boss.armor).max(1);
        if boss.hit_point <= 0 {
            return Some(player.cost);
        }
        player.hit_point -= (boss.damage - player.armor).max(1);
        if player.hit_point <= 0 {
            return None;
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret = get_all_players()
        .iter()
        .map(|p| fight(p).unwrap_or(10000))
        .min()
        .unwrap_or(-1);

    format_result!(ret)
}

fn fight1(player: &Player) -> Option<isize> {
    let mut player = player.clone();
    let mut boss = BOSS.clone();
    loop {
        boss.hit_point -= (player.damage - boss.armor).max(1);
        if boss.hit_point <= 0 {
            return None;
        }
        player.hit_point -= (boss.damage - player.armor).max(1);
        if player.hit_point <= 0 {
            println!(
                "boss win with cost: {} damage: {} armor: {}",
                player.cost, player.damage, player.armor
            );
            return Some(player.cost);
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret = get_all_players()
        .iter()
        .map(|p| fight1(p).unwrap_or(-1))
        .max()
        .unwrap_or(-1);

    format_result!(ret)
}
