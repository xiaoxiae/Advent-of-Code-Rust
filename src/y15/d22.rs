use crate::util::Day;
use std::cmp::PartialEq;
use std::mem;

pub struct D22;

#[derive(Clone, Debug)]
struct Unit {
    damage: isize,
    health: isize,
}

impl Unit {
    fn new(damage: isize, health: isize) -> Self {
        Self { damage, health }
    }
}

fn parse_boss(input: &str) -> Unit {
    let mut damage = 0;
    let mut health = 0;

    for line in input.lines() {
        if line.starts_with("Hit Points:") {
            health = line[11..].trim().parse().expect("Invalid health");
        } else if line.starts_with("Damage:") {
            damage = line[8..].trim().parse().expect("Invalid damage");
        }
    }

    Unit::new(damage, health)
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
enum Spell {
    Missile {
        cost: isize,
        damage: isize,
    },
    Drain {
        cost: isize,
        damage: isize,
        heal: isize,
    },

    Shield {
        cost: isize,
        armor: isize,
        duration: usize,
    },
    Poison {
        cost: isize,
        damage: isize,
        duration: usize,
    },
    Recharge {
        cost: isize,
        recharge: isize,
        duration: usize,
    },
}

fn attack_and_tick(
    boss: &mut Unit,
    player_health: &mut isize,
    player_mana: &mut isize,
    mut player_damage: isize,
    turn: bool,
    effects: &mut Vec<Spell>,
) {
    let mut boss_damage = boss.damage;

    effects.retain_mut(|effect| {
        match effect {
            Spell::Shield {
                duration, armor, ..
            } => {
                boss_damage = (boss_damage - *armor).max(1); // always deal at least 1 damage
                *duration -= 1;
                *duration > 0
            }
            Spell::Poison {
                duration, damage, ..
            } => {
                player_damage += *damage;
                *duration -= 1;
                *duration > 0
            }
            Spell::Recharge {
                duration, recharge, ..
            } => {
                *player_mana += *recharge;
                *duration -= 1;
                *duration > 0
            }
            _ => panic!("Invalid effect {:?}!", effect),
        }
    });

    if turn {
        boss.health -= player_damage;
    } else {
        boss.health -= player_damage;
        *player_health -= boss_damage;
    }
}

fn fight(
    boss: &Unit,
    mut health: isize,
    turn: bool,
    mana: isize,
    spent: isize,
    spells: &Vec<Spell>,
    effects: &Vec<Spell>,
    best_spent: &mut isize,
    hard: bool,
) {
    // prune!
    if spent >= *best_spent {
        return;
    }

    if hard && turn {
        health -= 1;
    }

    // if boss died, we win; otherwise we lose :(
    if boss.health <= 0 {
        if *best_spent > spent {
            *best_spent = spent;
        }

        return;
    } else if health <= 0 {
        return;
    }

    // boss' turn
    if !turn {
        let mut boss = boss.clone();
        let mut health = health.clone();
        let mut mana = mana.clone();
        let mut effects = effects.clone();

        attack_and_tick(&mut boss, &mut health, &mut mana, 0, turn, &mut effects);

        fight(
            &boss, health, !turn, mana, spent, spells, &effects, best_spent, hard,
        );

        return;
    }

    // player's turn
    'outer: for spell in spells {
        let mut boss = boss.clone();
        let mut health = health.clone();
        let mut mana = mana.clone();
        let mut effects = effects.clone();

        let mut player_damage = 0;
        let mut spell_cost = 0;
        let mut effect: Option<Spell> = None;

        match spell {
            Spell::Missile { damage, cost } if cost <= &mana => {
                player_damage += damage;

                spell_cost += *cost;
            }
            Spell::Drain { damage, heal, cost } if cost <= &mana => {
                player_damage += damage;
                health += heal;

                spell_cost += *cost;
            }
            Spell::Shield { cost, .. } if cost <= &mana => {
                effect = Option::from(spell.clone());

                spell_cost += *cost;
            }
            Spell::Poison { cost, .. } if cost <= &mana => {
                effect = Option::from(spell.clone());

                spell_cost += *cost;
            }
            Spell::Recharge { cost, .. } if cost <= &mana => {
                effect = Option::from(spell.clone());

                spell_cost += *cost;
            }
            _ => continue,
        };

        attack_and_tick(
            &mut boss,
            &mut health,
            &mut mana,
            player_damage,
            turn,
            &mut effects,
        );

        if let Some(e) = effect {
            for ongoing_effect in &effects {
                if mem::discriminant(&e) == mem::discriminant(ongoing_effect) {
                    continue 'outer;
                }
            }

            effects.push(e);
        }

        fight(
            &boss,
            health,
            !turn,
            mana - spell_cost,
            spent + spell_cost,
            spells,
            &effects,
            best_spent,
            hard,
        );
    }
}

fn solve(input: &str, hard: bool) -> Option<String> {
    let boss = parse_boss(input);

    let spells = vec![
        Spell::Missile {
            cost: 53,
            damage: 4,
        },
        Spell::Drain {
            cost: 73,
            damage: 2,
            heal: 2,
        },
        Spell::Shield {
            cost: 113,
            armor: 7,
            duration: 6,
        },
        Spell::Poison {
            cost: 173,
            damage: 3,
            duration: 6,
        },
        Spell::Recharge {
            cost: 229,
            recharge: 101,
            duration: 5,
        },
    ];

    let mut lowest_mana_spent = isize::MAX;

    fight(
        &boss,
        50,
        true,
        500,
        0,
        &spells,
        &Vec::new(),
        &mut lowest_mana_spent,
        hard,
    );

    Option::from(lowest_mana_spent.to_string())
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        solve(input, false)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        solve(input, true)
    }
}
