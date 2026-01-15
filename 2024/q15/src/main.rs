use std::fs;

#[derive(Debug)]
enum Objects {
    Robot,
    Box,
    Wall,
    Space,
}

fn main() {
    let file_name = "input/example";
    // let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let mut map = Vec::new();
    let mut robot_position = (0i32, 0i32);
    let mut moves = Vec::new();
    let mut map_deciphered = false;

    for line in contents.lines() {
        if line.is_empty() {
            map_deciphered = true;
            continue;
        }
        if map_deciphered {
            moves.extend(
                line.chars()
                    .map(|e| match e {
                        '<' => (0, -1),
                        '^' => (-1, 0),
                        '>' => (0, 1),
                        _ => (1, 0),
                    })
                    .collect::<Vec<_>>(),
            );
        } else {
            map.push(
                line.chars()
                    .enumerate()
                    .map(|(index, e)| match e {
                        '@' => {
                            robot_position = (map.len() as i32, index as i32);
                            Objects::Robot
                        }
                        '.' => Objects::Space,
                        'O' => Objects::Box,
                        _ => Objects::Wall,
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }

    for m in moves {
        let next_position = (robot_position.0 + m.0, robot_position.1 + m.1);

        match map[next_position.0 as usize][next_position.1 as usize] {
            Objects::Space => {
                map[next_position.0 as usize][next_position.1 as usize] = Objects::Robot;
                map[robot_position.0 as usize][robot_position.1 as usize] = Objects::Space;
                robot_position = next_position;
            }
            Objects::Box => {
                let mut final_position = next_position;
                loop {
                    final_position = (final_position.0 + m.0, final_position.1 + m.1);
                    match map[final_position.0 as usize][final_position.1 as usize] {
                        Objects::Space => break,
                        Objects::Wall => break,
                        _ => (),
                    }
                }
                if let Objects::Space = map[final_position.0 as usize][final_position.1 as usize] {
                    map[next_position.0 as usize][next_position.1 as usize] = Objects::Robot;
                    map[robot_position.0 as usize][robot_position.1 as usize] = Objects::Space;
                    map[final_position.0 as usize][final_position.1 as usize] = Objects::Box;
                    robot_position = next_position;
                }
            }
            Objects::Wall => {}
            _ => (),
        }
    }
    println!("{map:?}");

    let mut result = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            if let Objects::Box = *obj {
                result += i * 100 + j;
            }
        }
    }

    println!("{result}");
}
