//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/20
//!
//! ⚠️ part 2 UNSOLVED: the original 20-2.py never terminated (debug-only infinite
//! loop, no real answer). Left as None — not translating an agent-invented
//! solution. Part 1 (800830848) matched Python.
use crate::util::Day;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

pub struct D20;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Broadcast,
    FlipFlop,
    Conjunction,
}

struct Module {
    name: String,
    kind: Kind,
    outputs: Vec<String>,
    // FlipFlop state
    state: Signal,
    // Conjunction last-received per input
    last_received: FxHashMap<String, Signal>,
}

impl Module {
    fn receive(&mut self, source: &str, signal: Signal) -> Vec<(String, String, Signal)> {
        match self.kind {
            Kind::Broadcast => self
                .outputs
                .iter()
                .map(|o| (self.name.clone(), o.clone(), signal))
                .collect(),
            Kind::FlipFlop => {
                if signal == Signal::High {
                    return Vec::new();
                }
                self.state = if self.state == Signal::High {
                    Signal::Low
                } else {
                    Signal::High
                };
                let s = self.state;
                self.outputs
                    .iter()
                    .map(|o| (self.name.clone(), o.clone(), s))
                    .collect()
            }
            Kind::Conjunction => {
                self.last_received.insert(source.to_string(), signal);

                let all_high = self.last_received.values().all(|v| *v == Signal::High);
                let out_sig = if all_high { Signal::Low } else { Signal::High };

                self.outputs
                    .iter()
                    .map(|o| (self.name.clone(), o.clone(), out_sig))
                    .collect()
            }
        }
    }
}

fn parse(input: &str) -> FxHashMap<String, Module> {
    let mut modules: FxHashMap<String, Module> = FxHashMap::default();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (src, dst) = line.split_once(" -> ").unwrap();

        let kind = match src.as_bytes()[0] {
            b'%' => Kind::FlipFlop,
            b'&' => Kind::Conjunction,
            _ => Kind::Broadcast,
        };

        let outputs: Vec<String> = dst.split(", ").map(|s| s.to_string()).collect();

        let name = if kind != Kind::Broadcast {
            src[1..].to_string()
        } else {
            src.to_string()
        };

        modules.insert(
            name.clone(),
            Module {
                name,
                kind,
                outputs,
                state: Signal::Low,
                last_received: FxHashMap::default(),
            },
        );
    }

    // fill in conjunction inputs
    let edges: Vec<(String, String)> = modules
        .iter()
        .flat_map(|(name, m)| m.outputs.iter().map(move |o| (name.clone(), o.clone())))
        .collect();

    for (name, out) in edges {
        if let Some(m) = modules.get_mut(&out) {
            if m.kind == Kind::Conjunction {
                m.last_received.insert(name, Signal::Low);
            }
        }
    }

    modules
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut modules = parse(input);

        let mut low_count: u64 = 0;
        let mut high_count: u64 = 0;

        for _ in 0..1000 {
            let mut queue: VecDeque<(String, String, Signal)> = VecDeque::new();
            queue.push_back(("button".to_string(), "broadcaster".to_string(), Signal::Low));

            // count the initial button -> broadcaster low pulse
            low_count += 1;

            while let Some((src, dst, sig)) = queue.pop_front() {
                let received = match modules.get_mut(&dst) {
                    Some(m) => m.receive(&src, sig),
                    None => continue,
                };

                for item in received {
                    match item.2 {
                        Signal::Low => low_count += 1,
                        Signal::High => high_count += 1,
                    }
                    queue.push_back(item);
                }
            }
        }

        Some((low_count * high_count).to_string())
    }

    fn solve_part2(&self, _input: &str) -> Option<String> {
        // The original 20-2.py never terminated (debug-only infinite loop, no real
        // answer). Not translating an agent-invented solution here — left unsolved
        // pending a manual implementation.
        None
    }
}
