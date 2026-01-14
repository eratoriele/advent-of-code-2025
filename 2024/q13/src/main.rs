use regex::Regex;
use std::fs;

#[derive(Debug)]
struct ClawMachine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn solve_machine(machine: &ClawMachine) -> Option<(u64, u64)> {
    let mut b_presses = machine.prize.0 / machine.button_b.0;
    let a_presses: u64;

    loop {
        let remaning_x = machine.prize.0 - b_presses * machine.button_b.0;
        if remaning_x.is_multiple_of(machine.button_a.0) {
            let a_presses_temp = remaning_x / machine.button_a.0;
            if b_presses * machine.button_b.1 + a_presses_temp * machine.button_a.1
                == machine.prize.1
            {
                a_presses = a_presses_temp;
                break;
            }
        }
        if b_presses > 0 {
            b_presses -= 1;
        } else {
            return None;
        }
    }
    Some((a_presses, b_presses))
}

fn main() {
    let file_name = "input/example";
    // let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let re_button = Regex::new(r"^Button [AB]: X\+(?<first>\d+), Y\+(?<second>\d+)$").unwrap();
    let re_prize = Regex::new(r"^Prize: X=(?<first>\d+), Y=(?<second>\d+)$").unwrap();
    let mut machines: Vec<ClawMachine> = Vec::new();
    for lines in contents.lines().collect::<Vec<_>>().chunks(4) {
        let cap_a = re_button.captures(lines[0]).unwrap();
        let cap_b = re_button.captures(lines[1]).unwrap();
        let prize = re_prize.captures(lines[2]).unwrap();

        machines.push(ClawMachine {
            button_a: (
                cap_a["first"].parse::<u64>().unwrap(),
                cap_a["second"].parse::<u64>().unwrap(),
            ),
            button_b: (
                cap_b["first"].parse::<u64>().unwrap(),
                cap_b["second"].parse::<u64>().unwrap(),
            ),
            prize: (
                10_000_000_000_000 + prize["first"].parse::<u64>().unwrap(),
                10_000_000_000_000 + prize["second"].parse::<u64>().unwrap(),
            ),
        });
    }

    let mut part1 = 0u64;
    for machine in machines {
        if let Some((a, b)) = solve_machine(&machine) {
            part1 += b + a * 3;
        }
    }
    println!("{part1}");
}
