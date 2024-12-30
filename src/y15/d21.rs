use crate::util::Day;
use itertools::Itertools;
use std::cmp::{max, min};

pub struct D21;

#[derive(Debug)]
struct Unit {
    damage: isize,
    armor: isize,
    health: isize,
}

impl Unit {
    fn new(damage: isize, armor: isize, health: isize) -> Self {
        Self {
            damage,
            armor,
            health,
        }
    }

    fn fight(&self, other: &Unit) -> bool {
        let self_real_damage = (self.damage - other.armor).max(1);
        let other_real_damage = (other.damage - self.armor).max(1);

        other.health / self_real_damage <= self.health / other_real_damage
    }
}

#[derive(Debug, Clone)]
struct Item {
    name: &'static str,
    cost: isize,
    damage: isize,
    armor: isize,
}

impl Item {
    fn new(name: &'static str, cost: isize, damage: isize, armor: isize) -> Self {
        Self {
            name,
            cost,
            damage,
            armor,
        }
    }
}

fn parse_boss(input: &str) -> Unit {
    let mut damage = 0;
    let mut armor = 0;
    let mut health = 0;

    for line in input.lines() {
        if line.starts_with("Hit Points:") {
            health = line[11..].trim().parse().expect("Invalid health");
        } else if line.starts_with("Damage:") {
            damage = line[8..].trim().parse().expect("Invalid damage");
        } else if line.starts_with("Armor:") {
            armor = line[7..].trim().parse().expect("Invalid armor");
        }
    }

    Unit::new(damage, armor, health)
}

fn solve(
    input: &str,
    fight: fn(&Unit, &Unit) -> bool,
    initial: isize,
    comparison: fn(isize, isize) -> isize,
) -> isize {
    let boss = parse_boss(input);

    let weapons = vec![
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ];

    let armors = vec![
        Item::new("No Armor", 0, 0, 0),
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ];

    let rings = vec![
        Item::new("No Ring 1", 0, 0, 0),
        Item::new("No Ring 2", 0, 0, 0),
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ];

    let mut cost = initial;

    for weapon in &weapons {
        for armor in &armors {
            for ring_combination in rings.iter().combinations(2) {
                let total_damage = weapon.damage
                    + armor.damage
                    + ring_combination
                        .iter()
                        .map(|ring| ring.damage)
                        .sum::<isize>();

                let total_armor = weapon.armor
                    + armor.armor
                    + ring_combination
                        .iter()
                        .map(|ring| ring.armor)
                        .sum::<isize>();

                let total_cost = weapon.cost
                    + armor.cost
                    + ring_combination.iter().map(|ring| ring.cost).sum::<isize>();

                let player = Unit::new(total_damage, total_armor, 100);

                if fight(&player, &boss) {
                    cost = comparison(cost, total_cost);
                }
            }
        }
    }

    cost
}

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let cost = solve(input, |player, boss| player.fight(boss), isize::MAX, min);

        Option::from(cost.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let cost = solve(input, |player, boss| boss.fight(player), isize::MIN, max);

        Option::from(cost.to_string())
    }
}
