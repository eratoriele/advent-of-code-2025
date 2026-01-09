use std::fs;

fn solve_equation(target: u64, value: u64, values: &[u64]) -> u64 {
    if values.is_empty() {
        value
    } else {
        let mut next_value = solve_equation(target, value + values[0], &values[1..]);
        if target != next_value {
            next_value = solve_equation(target, value * values[0], &values[1..]);
        }
        if target != next_value {
            next_value = solve_equation(
                target,
                (value.to_string() + &values[0].to_string())
                    .parse()
                    .unwrap(),
                &values[1..],
            );
        }
        next_value
    }
}

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in contents.lines() {
        let split = line.split(": ").collect::<Vec<_>>();
        equations.push((
            split[0].parse().unwrap(),
            split[1]
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|&e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        ));
    }

    println!(
        "{}",
        equations.iter().fold(0, |acc, e| {
            if e.0 == solve_equation(e.0, e.1[0], &e.1[1..]) {
                acc + e.0
            } else {
                acc
            }
        })
    );
}
