use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

fn main() {
    // let input_file = "input/example";
    let input_file = "input/input";

    let contents = fs::read_to_string(input_file).expect("Input file expected");
    let mut start_point = (0, 0);
    let mut end_point = (0, 0);
    let map = contents
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start_point = (i, j);
                        0
                    }
                    'E' => {
                        end_point = (i, j);
                        0
                    }
                    '#' => 1,
                    _ => 0,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut dists = HashMap::new();
    let mut pq = BinaryHeap::new();

    pq.push((end_point, 0));
    dists.insert(end_point, 0);

    while let Some((pos, dist)) = pq.pop() {
        if dist > *dists.get(&pos).unwrap_or(&u64::MAX) {
            continue;
        }
        let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for m in moves {
            let next_pos = (
                pos.0.saturating_add_signed(m.0 as isize),
                pos.1.saturating_add_signed(m.1 as isize),
            );
            if map[next_pos.0][next_pos.1] != 1 {
                let next_cost = dist + 1;
                if next_cost < *dists.get(&next_pos).unwrap_or(&u64::MAX) {
                    dists.insert(next_pos, next_cost);
                    pq.push((next_pos, next_cost));
                }
            }
        }
    }

    let mut cheats = HashSet::new();
    for i in 0..=20 {
        for j in 0..=(20 - i) {
            let i = i as isize;
            let j = j as isize;
            cheats.insert((i, j));
            cheats.insert((-i, j));
            cheats.insert((i, -j));
            cheats.insert((-i, -j));
        }
    }
    let mut time_saves = HashMap::new();
    for (&(x, y), &curr_dist) in dists.iter() {
        let mut cheat_destinations = HashSet::new();
        for c in &cheats {
            let next_pos = (x.saturating_add_signed(c.0), y.saturating_add_signed(c.1));
            let cheat_cost = (c.0.abs() + c.1.abs()) as u64;
            if let Some(&cheated_point) = dists.get(&(next_pos.0, next_pos.1))
                && cheated_point + cheat_cost < curr_dist
                && cheat_destinations.insert(next_pos)
            {
                let time_save = curr_dist - cheated_point - cheat_cost;
                time_saves
                    .entry(time_save)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }
    println!("{time_saves:#?}");
    println!(
        "part2: {}",
        time_saves
            .iter()
            .fold(0, |acc, e| { if *e.0 >= 100 { acc + *e.1 } else { acc } })
    );
}
