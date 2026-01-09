use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let guard_movement = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut map: Vec<Vec<bool>> = Vec::new();
    let mut blocks: Vec<(i16, i16)> = Vec::new();
    let mut guard_spawn = (0, 0);
    let mut guard_direction = 0;
    for (line_index, line) in contents.lines().enumerate() {
        map.push(vec![false; line.chars().count()]);
        for (char_index, char) in line.chars().enumerate() {
            match char {
                '#' => blocks.push((line_index as i16, char_index as i16)),
                '^' => {
                    guard_spawn = (line_index as i16, char_index as i16);
                    map[line_index][char_index] = true;
                }
                _ => (),
            }
        }
    }

    let mut guard = guard_spawn;
    loop {
        let next_guard_point = (
            guard.0 + guard_movement[guard_direction].0,
            guard.1 + guard_movement[guard_direction].1,
        );
        println!("{next_guard_point:?}");
        if !(0..(map[0].len() as i16)).contains(&next_guard_point.0)
            || !(0..(map.len() as i16)).contains(&next_guard_point.1)
        {
            break;
        } else if blocks.contains(&next_guard_point) {
            guard_direction = (guard_direction + 1) % guard_movement.len();
        } else {
            guard = next_guard_point;
            map[guard.0 as usize][guard.1 as usize] = true;
        }
    }

    let mut visited_nodes: Vec<(i16, i16)> = Vec::new();
    for (x_i, row) in map.iter().enumerate() {
        for (y_i, &b) in row.iter().enumerate() {
            if b {
                visited_nodes.push((x_i as i16, y_i as i16));
            }
        }
    }
    println!("{}", visited_nodes.len());
    let mut results = 0u32;

    for (index, &(i, j)) in visited_nodes.iter().enumerate() {
        println!("{index}");
        let mut guard = guard_spawn;
        guard_direction = 0;
        let mut new_blocks = blocks.to_vec();
        new_blocks.push((i, j));

        // block location, approached from
        let mut cache: HashSet<((i16, i16), (i16, i16))> = HashSet::new();
        loop {
            let next_guard_point = (
                guard.0 + guard_movement[guard_direction].0,
                guard.1 + guard_movement[guard_direction].1,
            );
            if !(0..(map[0].len() as i16)).contains(&next_guard_point.0)
                || !(0..(map.len() as i16)).contains(&next_guard_point.1)
            {
                break;
            } else if new_blocks.contains(&next_guard_point) {
                if !cache.insert((guard, next_guard_point)) {
                    results += 1;
                    break;
                }
                guard_direction = (guard_direction + 1) % guard_movement.len();
            } else {
                guard = next_guard_point;
                // map[guard.0 as usize][guard.1 as usize] = true;
            }
        }
    }

    println!("{results}");

    // println!(
    //     "{}",
    //     map.iter()
    //         .flatten()
    //         .fold(0, |acc, e| acc + if *e { 1 } else { 0 })
    // );
}
