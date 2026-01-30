use std::{collections::HashMap, fs};

fn compare_slices(current: &[char], target: &[char]) -> bool {
    if current.len() > target.len() {
        false
    } else {
        for i in 0..current.len() {
            if current[i] != target[i] {
                return false;
            }
        }
        true
    }
}

fn dfs(
    goal: &[char],
    action: &[char],
    options: &[Vec<char>],
    current: &mut Vec<char>,
    cache: &mut HashMap<Vec<char>, u64>,
) -> Option<u64> {
    current.extend_from_slice(action);
    if !compare_slices(current, goal) {
        None
    } else if current.len() == goal.len() {
        Some(1)
    } else {
        if let Some(&sc) = cache.get(current) {
            return Some(sc);
        }
        let mut total_score = 0;
        for opt in options {
            let result = dfs(goal, opt, options, current, cache);
            if let Some(ns) = result {
                total_score += ns;
            }
            current.truncate(current.len() - opt.len());
        }
        cache.insert(current.to_vec(), total_score);
        Some(total_score)
    }
}

fn main() {
    // let input_file = "input/example";
    let input_file = "input/input";

    let contents = fs::read_to_string(input_file).expect("Input file expected");
    let mut lines_iter = contents.lines();

    let towels = lines_iter
        .next()
        .unwrap()
        .split(", ")
        .map(|t| t.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // skip the newline
    lines_iter.next();
    let goals = lines_iter
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1 = goals.iter().fold(0, |acc, g| {
        let score = dfs(g, &[], &towels, &mut Vec::new(), &mut HashMap::new());
        if let Some(sc) = score { acc + sc } else { acc }
    });
    println!("{part1}",);
}
