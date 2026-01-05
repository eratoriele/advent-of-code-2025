use std::fs;

#[derive(Debug)]
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
    buttons: Vec<Vec<u8>>,
}

fn main() {
    let file_path = "./input/example";
    // let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut machines: Vec<Machine> = Vec::new();
    for line in contents.lines() {
        let mut machine = Machine {
            lights: Vec::new(),
            buttons: Vec::new(),
        };
        let split: Vec<&str> = line.split_whitespace().collect();
        let mut split_iter = split.iter();

        // parse lights
        let lights = split_iter.next().unwrap();
        for light in lights.chars() {
            if light != '[' && light != ']' {
                machine.lights.push(match light {
                    '#' => LightState::On,
                    _ => LightState::Off,
                });
            }
        }

        // parse buttons
        for button in split_iter.take(line.split_whitespace().count() - 2) {
            let activators: String = button
                .chars()
                .skip(1)
                .take(button.chars().count() - 2)
                .collect();
            let activators: Vec<&str> = activators.split(',').collect();
            machine.buttons.push(
                activators
                    .iter()
                    .map(|e| e.parse::<u8>().unwrap())
                    .collect(),
            );
        }

        // parse joltages
        // let j = split_iter.next().unwrap();

        machines.push(machine);
    }

    println!("{machines:#?}");
}
