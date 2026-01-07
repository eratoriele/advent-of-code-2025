use std::{collections::HashMap, fs};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Device {
    Start,
    Node(String),
    End,
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

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut devices: HashMap<Device, Vec<Device>> = HashMap::new();
    for line in contents.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        devices.insert(
            match &split[0][..(split[0].len() - 1)] {
                "you" => Device::Start,
                node => Device::Node(node.to_string()),
            },
            split[1..]
                .iter()
                .map(|e| match *e {
                    "you" => Device::Start,
                    "out" => Device::End,
                    node => Device::Node(node.to_string()),
                })
                .collect::<Vec<_>>(),
        );
    }

    let (_, i, part1) = dfs_part1(&Device::Start, &devices, 0, 0);
    println!("part1: {i}, {part1}");
}
