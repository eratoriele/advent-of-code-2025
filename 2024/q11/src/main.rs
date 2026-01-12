use std::{collections::HashMap, fs};

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();
    let mut stones: HashMap<u64, u64> = contents
        .split_whitespace()
        .map(|e| (e.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();

    println!("{stones:?}");

    let mut cache: HashMap<u64, (u64, Option<u64>)> = HashMap::new();
    for i in 0..75 {
        println!("{i}");
        let mut new_stones = HashMap::new();
        for (&stone, &count) in &stones {
            if let Some(pre_computed) = cache.get(&stone) {
                new_stones
                    .entry(pre_computed.0)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                if let Some(pre_computed_2) = pre_computed.1 {
                    new_stones
                        .entry(pre_computed_2)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                }
            } else if stone == 0 {
                new_stones
                    .entry(1)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                cache.insert(stone, (1, None));
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let s = (
                    stone_str[..stone_str.len() / 2].parse::<u64>().unwrap(),
                    stone_str[stone_str.len() / 2..].parse::<u64>().unwrap(),
                );
                new_stones
                    .entry(s.0)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                new_stones
                    .entry(s.1)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                cache.insert(stone, (s.0, Some(s.1)));
            } else {
                new_stones
                    .entry(stone * 2024)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                cache.insert(stone, (stone * 2024, None));
            }
        }
        stones = new_stones;
    }
    println!(
        "part2: {}",
        stones.iter().fold(0, |acc, (_, count)| acc + count)
    );
}
