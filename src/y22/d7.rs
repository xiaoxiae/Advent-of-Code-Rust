use crate::util::Day;
use std::collections::HashMap;

pub struct D7;

type FileSystem = HashMap<String, Node>;

enum Node {
    File(usize),
    Directory(FileSystem),
}

impl Node {
    fn as_directory_mut(&mut self) -> Option<&mut FileSystem> {
        match self {
            Node::Directory(subdir) => Some(subdir),
            _ => None,
        }
    }
}

fn build_filesystem(input: &str) -> FileSystem {
    let mut pos = Vec::new();
    let mut tree: FileSystem = HashMap::new();

    for command in input.lines() {
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.as_slice() {
            [_, "ls"] => continue,
            [_, "cd", ".."] => { pos.pop(); }
            [_, "cd", "/"] => { pos.clear(); }
            [_, "cd", dir] => { pos.push((*dir).to_string()); }
            [size, _] if *size == "dir" => {
                let mut current = &mut tree;
                for p in &pos {
                    current = current
                        .entry(p.clone())
                        .or_insert_with(|| Node::Directory(HashMap::new()))
                        .as_directory_mut()
                        .unwrap();
                }
            }
            [size, name] => {
                if let Ok(file_size) = size.parse::<usize>() {
                    let mut current = &mut tree;
                    for p in &pos {
                        current = current
                            .entry(p.clone())
                            .or_insert_with(|| Node::Directory(HashMap::new()))
                            .as_directory_mut()
                            .unwrap();
                    }
                    current.insert(name.to_string(), Node::File(file_size));
                }
            }
            _ => continue,
        }
    }

    tree
}

fn sum_directories(root: &FileSystem, directories: &mut HashMap<String, usize>, path: String) -> usize {
    let mut total = 0;

    for (name, node) in root {
        match node {
            Node::File(size) => total += size,
            Node::Directory(subdir) => {
                let subpath = format!("{}/{}", path, name);
                total += sum_directories(subdir, directories, subpath.clone());
            }
        }
    }

    directories.insert(path, total);
    total
}


impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let tree = build_filesystem(input);

        let mut directories = HashMap::new();
        sum_directories(&tree, &mut directories, "".to_string());

        let total: usize = directories.values().filter(|&&v| v < 100_000).sum();

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let tree = build_filesystem(input);

        let mut directories = HashMap::new();
        sum_directories(&tree, &mut directories, "".to_string());

        let total_used = directories[""];
        let to_delete = 30_000_000 - (70_000_000 - total_used);
        let current_best = directories.values().filter(|&&v| v > to_delete).min().copied().unwrap_or(0);

        Option::from(current_best.to_string())
    }
}
