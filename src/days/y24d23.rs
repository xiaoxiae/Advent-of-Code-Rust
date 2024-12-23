use crate::util::Day;
use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct Y24D23;

fn get_largest_clique(
    clique: &Vec<u16>,
    remaining: &[u16],
    max_clique_size: usize,
    graph: &HashMap<u16, HashSet<u16>>,
) -> Vec<u16> {
    if max_clique_size == clique.len() {
        return clique.clone();
    }

    if remaining.len() == 0 {
        return clique.clone();
    }

    let u = remaining[0];
    let u_neighbours = graph.get(&u).unwrap();

    let mut valid = true;
    for &v in clique {
        if !u_neighbours.contains(&v) {
            valid = false;
            break;
        }
    }

    let mut o1 = vec![];
    if valid {
        let mut new = clique.clone();
        new.push(u);

        o1 = get_largest_clique(
            &new,
            &remaining[1..],
            max_clique_size.min(u_neighbours.len()),
            graph,
        );
    }

    let o2 = get_largest_clique(clique, &remaining[1..], max_clique_size, graph);

    if o1.len() > o2.len() {
        o1
    } else {
        o2
    }
}

impl Day for Y24D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut graph: HashMap<&str, HashSet<&str>> = HashMap::default();

        input
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .for_each(|(u, v)| {
                graph.entry(u).or_insert_with(HashSet::default).insert(v);
                graph.entry(v).or_insert_with(HashSet::default).insert(u);
            });

        let mut triples: HashSet<(&str, &str, &str)> = HashSet::default();

        for node in graph.keys() {
            for u in graph.get(node).unwrap() {
                for v in graph.get(node).unwrap() {
                    if !(node < u && u < v) {
                        continue;
                    }

                    if graph.contains_key(u) && graph.get(u).unwrap().contains(v) {
                        triples.insert((*node, *u, *v));
                    }
                }
            }
        }

        let total = triples
            .iter()
            .filter(|(u, v, w)| u.contains('t') || v.contains('t') || w.contains('t'))
            .count();

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut graph: HashMap<u16, HashSet<u16>> = HashMap::default();

        input
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .for_each(|(u_str, v_str)| {
                let u_chars: Vec<_> = u_str.chars().map(|c| c as u16).collect();
                let v_chars: Vec<_> = v_str.chars().map(|c| c as u16).collect();

                let u = (u_chars[0] << 8) + u_chars[1];
                let v = (v_chars[0] << 8) + v_chars[1];

                graph.entry(u).or_insert_with(HashSet::default).insert(v);
                graph.entry(v).or_insert_with(HashSet::default).insert(u);
            });

        let mut nodes: Vec<_> = graph.keys().map(|&v| v).collect();
        nodes.sort();

        let mut clique = get_largest_clique(&vec![], &nodes[..], usize::MAX, &graph);
        clique.sort();

        Option::from(
            clique
                .iter()
                .map(|s| {
                    vec![(s >> 8) as u8 as char, (s & 0xFF) as u8 as char]
                        .iter()
                        .collect::<String>()
                })
                .join(","),
        )
    }
}
