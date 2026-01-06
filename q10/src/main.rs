use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum LightState {
    On,
    Off,
}
impl LightState {
    fn toggle(&mut self) {
        *self = match self {
            Self::On => LightState::Off,
            Self::Off => LightState::On,
        }
    }
}

#[derive(Debug)]
struct Machine {
    lights: Vec<LightState>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn dfs_part1(
    curr_lights: &mut [LightState],
    lights: &[LightState],
    i: usize,
    buttons: &[Vec<usize>],
    cache: &mut HashMap<(Vec<LightState>, usize), i64>,
) -> i64 {
    if curr_lights == lights {
        return 0;
    }
    // if a button was pressed multiple times this solution can not be valid
    if i == buttons.len() {
        return -1;
    }

    let key = (curr_lights.to_vec(), i);
    if let Some(c) = cache.get(&key) {
        return *c;
    }

    let mut result = i64::MAX - 1;
    for j in i..buttons.len() {
        for &k in &buttons[j] {
            curr_lights[k].toggle();
        }
        // brute force pressing all buttons, get the minimum
        let r = 1 + dfs_part1(curr_lights, lights, j + 1, buttons, cache);
        if r > 0 {
            result = result.min(r);
        }
        for &k in &buttons[j] {
            curr_lights[k].toggle();
        }
    }

    cache.insert(key, result);

    result
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut machines: Vec<Machine> = Vec::new();
    for line in contents.lines() {
        let mut machine = Machine {
            lights: Vec::new(),
            buttons: Vec::new(),
            joltages: Vec::new(),
        };
        let split = line.split_whitespace().collect::<Vec<_>>();
        // let mut split_iter = split.iter();

        // parse lights
        machine.lights = split[0]
            .chars()
            .skip(1)
            .take(split[0].len() - 2)
            .map(|e| match e {
                '#' => LightState::On,
                _ => LightState::Off,
            })
            .collect::<Vec<LightState>>();

        // parse buttons
        machine.buttons = split[1..split.len() - 1]
            .iter()
            .map(|e| {
                e.chars()
                    .skip(1)
                    .take(e.len() - 2)
                    .collect::<String>()
                    .split(',')
                    .map(|e2| e2.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();

        // let j = split.iter().last().unwrap();
        machine.joltages = split[split.len() - 1][1..(split[split.len() - 1].len() - 1)]
            .split(',')
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        machines.push(machine);
    }

    println!("{machines:#?}");
    let mut total1 = 0;
    for m in &machines {
        let mut lights = vec![LightState::Off; m.lights.len()];
        total1 += dfs_part1(&mut lights, &m.lights, 0, &m.buttons, &mut HashMap::new());
    }
    println!("{total1}");
}
