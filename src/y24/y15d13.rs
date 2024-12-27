use crate::util::Day;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap as HashMap, FxHashMap, FxHashSet as HashSet};

pub struct Y15D13;

fn parse(input: &str) -> (HashSet<char>, HashMap<(char, char), isize>) {
    let re = Regex::new( r"(?P<name1>\w+) would (?P<action>gain|lose) (?P<value>\d+) happiness units by sitting next to (?P<name2>\w+)\." ).unwrap();

    let mut edges = HashMap::default();
    let mut vertices = HashSet::default();

    for line in input.lines() {
        if let Some(caps) = re.captures(line.trim()) {
            let name1 = &caps["name1"];
            let name2 = &caps["name2"];

            let action = &caps["action"];
            let value: isize = caps["value"].parse().unwrap();

            let adjusted_value = if action == "gain" { value } else { -value };

            let u = name1.chars().nth(0).unwrap();
            let v = name2.chars().nth(0).unwrap();

            edges.insert((u, v), adjusted_value);

            vertices.insert(u);
            vertices.insert(v);
        }
    }

    (vertices, edges)
}

fn happiness(people: Vec<&char>, edges: &HashMap<(char, char), isize>) -> isize {
    let mut total = 0;

    for i in 0..people.len() {
        let u = people[i];
        let v = people[(i + 1) % people.len()];

        total += edges[&(*u, *v)];
        total += edges[&(*v, *u)];
    }

    total
}

impl Day for Y15D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (vertices, edges) = parse(input);
        
        let max = vertices
            .iter()
            .permutations(vertices.len())
            .map(|perm| happiness(perm, &edges))
            .max().unwrap();

        Option::from(max.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut vertices, mut edges) = parse(input);

        for v in &vertices {
            edges.insert((*v, '0'), 0);
            edges.insert(('0', *v), 0);
        }
        
        vertices.insert('0');  // me!

        let max = vertices
            .iter()
            .permutations(vertices.len())
            .map(|perm| happiness(perm, &edges))
            .max().unwrap();

        Option::from(max.to_string())
    }
}
