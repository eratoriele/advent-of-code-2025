use std::{collections::HashMap, fs};

fn main() {
    // let input_file = "input/example";
    let input_file = "input/input";

    let contents = fs::read_to_string(input_file).expect("Input file expected");

    let mut change_results: HashMap<[Option<i8>; 4], [Option<u16>; 1629]> = HashMap::new();
    for (i, line) in contents.lines().enumerate() {
        let mut secret = line.parse::<u64>().unwrap();
        let mut last_changes: [Option<i8>; 4] = [None; 4];
        let mut last_digit = (secret % 10) as u16;
        for _ in 0..2000 {
            secret = ((secret * 64) ^ secret) % 16777216;
            secret = ((secret / 32) ^ secret) % 16777216;
            secret = ((secret * 2048) ^ secret) % 16777216;

            last_changes.copy_within(1..4, 0);
            last_changes[3] = Some((secret % 10) as i8 - last_digit as i8);
            last_digit = (secret % 10) as u16;
            if last_changes.iter().all(|e| e.is_some()) {
                change_results
                    .entry(last_changes)
                    .and_modify(|e| {
                        if e[i].is_none() {
                            e[i] = Some(last_digit)
                        }
                    })
                    .or_insert({
                        let mut new_arr = [None; 1629];
                        new_arr[i] = Some(last_digit);
                        new_arr
                    });
            }
        }
    }

    let mut highest_sum = 0u64;
    let mut highest_seq = [None; 4];
    for (seq, res) in change_results {
        let sum = res.iter().fold(0u64, |acc, e| {
            if let Some(price) = e {
                acc + *price as u64
            } else {
                acc
            }
        });
        if sum > highest_sum {
            highest_sum = sum;
            highest_seq = seq;
        }
    }

    println!("{highest_seq:?} => {highest_sum}");
}
