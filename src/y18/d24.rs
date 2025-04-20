use std::cmp::max;
use crate::util::Day;
use regex::Regex;

pub struct D24;

#[derive(Debug, PartialEq, Clone)]
enum AttackType {
    Bludgeoning,
    Radiation,
    Slashing,
    Fire,
    Cold,
}

#[derive(Debug, PartialEq, Clone)]
enum GroupType {
    System,
    Infection,
}

#[derive(Debug, PartialEq, Clone)]
struct Group {
    kind: GroupType,

    count: isize,
    health: isize,

    attack: isize,
    attack_type: AttackType,
    initiative: isize,

    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
}

impl Group {
    fn effective_power(&self) -> isize {
        self.count * self.attack
    }

    /// Damage the group would inflict on another group.
    fn calculate_damage_inflicted(&self, other: &Group) -> isize {
        let mut multiplier = 1;

        if other.weaknesses.contains(&self.attack_type) {
            multiplier = 2;
        } else if other.immunities.contains(&self.attack_type) {
            multiplier = 0;
        }

        multiplier * self.effective_power()
    }
}

impl AttackType {
    fn from_str(input: &str) -> AttackType {
        match input.as_ref() {
            "bludgeoning" => AttackType::Bludgeoning,
            "radiation" => AttackType::Radiation,
            "slashing" => AttackType::Slashing,
            "fire" => AttackType::Fire,
            "cold" => AttackType::Cold,
            _ => panic!("Invalid attack type '{}'", input),
        }
    }
}

fn extract_attributes(regex: &Regex, line: &str) -> Vec<AttackType> {
    regex
        .captures(line)
        .and_then(|captures| captures.get(1))
        .map(|matched| {
            matched
                .as_str()
                .split(", ")
                .map(AttackType::from_str)
                .collect()
        })
        .unwrap_or_default()
}

fn parse(input: &str) -> Vec<Group> {
    let line_re = Regex::new(r"(\d+) units each with (\d+) hit points(.*)with an attack that does (\d+) (\w+) damage at initiative (\d+)")
        .unwrap();

    let immune_to_re = Regex::new(r"immune to ([^;\n]+)[;)]").unwrap();
    let weak_to_re = Regex::new(r"weak to ([^;\n]+)[;)]").unwrap();

    let mut groups = vec![];

    for (i, part) in input.split("\n\n").enumerate() {
        let mut lines = part.lines();
        lines.next();

        for line in lines {
            let matches = line_re.captures(line).unwrap();

            groups.push(Group {
                kind: if i == 0 {
                    GroupType::System
                } else {
                    GroupType::Infection
                },
                count: matches[1].parse().unwrap(),
                health: matches[2].parse().unwrap(),
                attack: matches[4].parse().unwrap(),
                attack_type: AttackType::from_str(&matches[5]),
                initiative: matches[6].parse().unwrap(),
                weaknesses: extract_attributes(&weak_to_re, line),
                immunities: extract_attributes(&immune_to_re, line),
            })
        }
    }

    groups
}

fn solve(groups: &mut Vec<Group>) -> (isize, Option<GroupType>) {
    // prevent infinite loops
    let mut someone_died;

    loop {
        someone_died = false;

        // target selection
        groups.sort_by(|a, b| {
            // first by effective power, then by initiative
            b.effective_power()
                .cmp(&a.effective_power())
                .then(b.initiative.cmp(&a.initiative))
        });

        // each target can be selected at most once
        let mut valid_targets = groups.iter().collect::<Vec<_>>();

        // for each group, pick a valid target that it deals the most damage to
        let mut attack_pairs: Vec<(usize, usize)> = vec![];
        for (i, group) in groups.iter().enumerate() {
            let mut enemies = valid_targets
                .iter()
                .filter(|target| target.kind != group.kind)
                .map(|&target| target)
                .collect::<Vec<_>>();

            // no targets -- skip
            if enemies.is_empty() {
                continue;
            }

            enemies.sort_by(|a, b| {
                // first by damage, then by other group's effective power
                group
                    .calculate_damage_inflicted(b)
                    .cmp(&group.calculate_damage_inflicted(a))
                    .then(b.effective_power().cmp(&a.effective_power()))
            });

            // if we actually deal damage, pick the target
            if group.calculate_damage_inflicted(enemies[0]) > 0 {
                attack_pairs.push((i, groups.iter().position(|g| g == enemies[0]).unwrap()));

                valid_targets.retain(|&target| target != enemies[0]);
            }
        }

        attack_pairs.sort_by(|(i, _), (j, _)| groups[*j].initiative.cmp(&groups[*i].initiative));

        // for each group, pick a valid target that it deals the most damage to
        // deal damage
        for (attacker_idx, defender_idx) in attack_pairs {
            let attacker = &groups[attacker_idx];
            let defender = &groups[defender_idx];

            let damage = attacker.calculate_damage_inflicted(defender);

            let defender = &mut groups[defender_idx];

            // kill enemies, making sure that the group has non-negative units
            let old_count = defender.count;
            defender.count = (defender.count - damage / defender.health).max(0);

            if old_count != defender.count {
                someone_died = true;
            }
        }

        groups.retain(|group| group.count > 0);

        if !(groups.iter().any(|g| g.kind == GroupType::Infection)
            && groups.iter().any(|g| g.kind == GroupType::System))
        {
            break;
        }

        // draw (they don't have the strength to kill one another)
        if !someone_died {
            break;
        }
    }

    let mut total = 0;
    let mut kind = None;
    for group in groups {
        total += group.count;

        if someone_died {
            kind = Some(group.kind.clone());
        }
    }

    (total, kind)
}

fn solve_with_boost(groups: &Vec<Group>, boost: isize) -> (isize, Option<GroupType>) {
    let mut groups = groups
        .iter()
        .map(|g| {
            let mut g = g.clone();

            if g.kind == GroupType::System {
                g.attack += boost;
            }

            g
        })
        .collect::<Vec<_>>();

    solve(&mut groups)
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut groups = parse(input);

        let (total, _) = solve(&mut groups);

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let groups = parse(input);

        let mut min_boost = 0;
        let mut max_boost = 1;

        loop {
            let (_, winner) = solve_with_boost(&groups, max_boost);

            if winner == Some(GroupType::System) {
                break;
            }

            min_boost = max_boost;
            max_boost *= 2;
        }

        while min_boost != max_boost {
            let boost = (min_boost + max_boost) / 2;

            let (total, winner) = solve_with_boost(&groups, boost);

            // note that == Some(GroupType::Infection) doesn't work because of stalemates (took a second to debug)
            if winner != Some(GroupType::System) {
                min_boost = boost + 1;
            } else {
                max_boost = boost;
            }
        }

        panic!();
    }
}
