use rustc_hash::FxHashMap as HashMap;

use crate::util::Day;

pub struct D11;

fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, HashMap<String, usize>) {
    let mut ids: HashMap<String, usize> = HashMap::default();

    let intern = |name: &str, ids: &mut HashMap<String, usize>| -> usize {
        let next = ids.len();
        *ids.entry(name.to_string()).or_insert(next)
    };

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::default();

    for line in input.lines() {
        let (src, dsts) = line.split_once(':').unwrap();
        let src = intern(src.trim(), &mut ids);

        let neighbours = dsts
            .split_whitespace()
            .map(|d| intern(d, &mut ids))
            .collect();

        graph.insert(src, neighbours);
    }

    (graph, ids)
}

fn solve_1(
    cache: &mut HashMap<usize, usize>,
    graph: &HashMap<usize, Vec<usize>>,
    current: usize,
    end: usize,
) -> usize {
    if let Some(&result) = cache.get(&current) {
        return result;
    }

    if current == end {
        return 1;
    }

    if graph.get(&current).is_none() {
        return 0;
    }

    let mut total = 0;

    for neighbour in graph.get(&current).unwrap() {
        total += solve_1(cache, graph, *neighbour, end);
    }

    cache.insert(current, total);

    total
}

fn solve_2(
    cache: &mut HashMap<(usize, bool, bool), usize>,
    graph: &HashMap<usize, Vec<usize>>,
    current: usize,
    end: usize,
    dac_id: usize,
    fft_id: usize,
    mut visited_dac: bool,
    mut visited_fft: bool,
) -> usize {
    if let Some(&result) = cache.get(&(current, visited_dac, visited_fft)) {
        return result;
    }

    if current == end && visited_dac && visited_fft {
        return 1;
    }

    if graph.get(&current).is_none() {
        return 0;
    }

    let mut total = 0;

    if current == dac_id {
        visited_dac = true;
    }

    if current == fft_id {
        visited_fft = true;
    }

    for neighbour in graph.get(&current).unwrap() {
        total += solve_2(
            cache,
            graph,
            *neighbour,
            end,
            dac_id,
            fft_id,
            visited_dac,
            visited_fft,
        );
    }

    cache.insert((current, visited_dac, visited_fft), total);

    total
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (graph, ids) = parse(input);

        let mut cache: HashMap<usize, usize> = HashMap::default();

        let start = *ids.get("you").unwrap();
        let end = *ids.get("out").unwrap();

        Some(solve_1(&mut cache, &graph, start, end).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (graph, ids) = parse(input);

        let mut cache: HashMap<(usize, bool, bool), usize> = HashMap::default();

        let start = *ids.get("svr").unwrap();
        let end = *ids.get("out").unwrap();

        Some(
            solve_2(
                &mut cache,
                &graph,
                start,
                end,
                *ids.get("dac").unwrap(),
                *ids.get("fft").unwrap(),
                false,
                false,
            )
            .to_string(),
        )
    }
}
