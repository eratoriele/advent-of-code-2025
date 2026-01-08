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

// fn dfs_part1(
//     current_device: &Device,
//     devices: &HashMap<Device, Vec<Device>>,
//     i: u32,
//     counter: usize,
// ) -> (bool, u32, usize) {
//     let curr_i = i + 1;
//     if *current_device == Device::End {
//         return (true, i + 1, counter + 1);
//     }
//     // Eliminate circular paths
//     if i as usize > devices.len() {
//         return (false, u32::MAX, counter);
//     }
//
//     let mut result = counter;
//     for dev in devices.get(current_device).unwrap() {
//         let (res, _iter, ctr) = dfs_part1(dev, devices, curr_i, counter);
//         if res {
//             result += ctr;
//         }
//     }
//     // println!("{current_device:?}, {curr_i}, {result}");
//     (true, curr_i, result)
// }

fn dfs_part2(
    current_device: &Device,
    devices: &HashMap<Device, Vec<Device>>,
    visited: &mut HashSet<Device>,
    dac_found: bool,
    fft_found: bool,
    cache: &mut HashMap<(Device, bool, bool), u64>,
) -> u64 {
    let dac = dac_found || *current_device == Device::Dac;
    let fft = fft_found || *current_device == Device::Fft;

    let key = (current_device.clone(), dac, fft);
    if let Some(&count) = cache.get(&key) {
        return count;
    }
    // Detect a cycle
    if visited.contains(current_device) {
        println!("cycle found! {current_device:?}");
        return if dac && fft { 1 } else { 0 };
    }
    if *current_device == Device::End {
        return if dac && fft { 1 } else { 0 };
    }

    let mut total_paths = 0;
    visited.insert(current_device.clone());

    for dev in devices.get(current_device).unwrap() {
        total_paths += dfs_part2(dev, devices, visited, dac, fft, cache);
    }
    visited.remove(current_device);

    cache.insert(key, total_paths);
    total_paths
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

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
    let mut visited = HashSet::new();
    let mut cache = HashMap::new();
    let p2 = dfs_part2(
        &Device::Start,
        &devices,
        &mut visited,
        false,
        false,
        &mut cache,
    );
    println!("part2: {p2}");
}
