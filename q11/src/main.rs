use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Device {
    Start,
    Dac,
    Fft,
    Node(String),
    End,
}
fn parse_device(inp: &str) -> Device {
    match inp {
        "svr" => Device::Start,
        "dac" => Device::Dac,
        "fft" => Device::Fft,
        "out" => Device::End,
        node => Device::Node(node.to_string()),
    }
}

fn dfs_part1(
    current_device: &Device,
    devices: &HashMap<Device, Vec<Device>>,
    i: u32,
    counter: usize,
) -> (bool, u32, usize) {
    let curr_i = i + 1;
    if *current_device == Device::End {
        return (true, i + 1, counter + 1);
    }
    // Eliminate circular paths
    if i as usize > devices.len() {
        return (false, u32::MAX, counter);
    }

    let mut result = counter;
    for dev in devices.get(current_device).unwrap() {
        let (res, _iter, ctr) = dfs_part1(dev, devices, curr_i, counter);
        if res {
            result += ctr;
        }
    }
    println!("{current_device:?}, {curr_i}, {result}");
    (true, curr_i, result)
}

fn dfs_part2<'a>(
    current_device: &'a Device,
    devices: &'a HashMap<Device, Vec<Device>>,
    counter: usize,
    dac_passed: bool,
    fft_passed: bool,
    visited: &mut Vec<&'a Device>,
) -> (bool, bool, bool, usize) {
    // Detect a cycle
    if visited.contains(&current_device) {
        // if !visited.insert(current_device) {
        // current_device was already in the set → cycle
        println!("{visited:?}, {current_device:?}");
        return (false, dac_passed, fft_passed, counter);
    }
    visited.push(current_device);
    let mut dac = dac_passed;
    let mut fft = fft_passed;
    match *current_device {
        Device::End => return (true, dac, fft, counter + 1),
        Device::Dac => dac = true,
        Device::Fft => fft = true,
        _ => (),
    }

    let mut result = counter;
    for dev in devices.get(current_device).unwrap() {
        // println!("{current_device:?} => {:?}, {result}", dev);
        let (res, dp, fp, ctr) = dfs_part2(dev, devices, result, dac, fft, visited);
        println!("{current_device:?} <- {:?}, {ctr}, {res} {dp} {fp}", dev);
        if dp {
            dac = dp;
        }
        if fp {
            fft = fp;
        }
        if res && dac && fft {
            result = ctr;
        }
        let removed = visited.remove(visited.len() - 1);
        match removed {
            Device::End => println!("{visited:?} {fft} {dac} {result} {ctr}"),
            Device::Fft => fft = false,
            Device::Dac => dac = false,
            _ => (),
        }
    }
    // println!("{current_device:?}, {curr_i}, {result}");
    // visited.remove(current_device);
    (true, dac, fft, result)
}

fn main() {
    let file_path = "./input/example";
    // let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut devices: HashMap<Device, Vec<Device>> = HashMap::new();
    for line in contents.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        devices.insert(
            parse_device(&split[0][..(split[0].len() - 1)]),
            split[1..]
                .iter()
                .map(|e| parse_device(e))
                .collect::<Vec<_>>(),
        );
    }

    // let (_, i, part1) = dfs_part1(&Device::Start, &devices, 0, 0);
    // println!("part1: {i}, {part1}");
    // let mut visited = HashSet::new();
    let mut visited = Vec::new();
    let (_, _, _, part2) = dfs_part2(&Device::Start, &devices, 0, false, false, &mut visited);
    println!("part2: {part2}");
}
