use regex::Regex;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;

// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn print_map(file: &mut File, map: &[[i32; WIDTH as usize]; HEIGHT as usize]) {
    for i in 0..map.len() {
        let mut line = "".to_string();
        for j in 0..map[0].len() {
            line += match map[i][j] {
                0 => " ",
                _ => "*",
            };
        }
        if let Err(e) = writeln!(file, "{line}") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let mut map = [[0; WIDTH as usize]; HEIGHT as usize];
    let re = Regex::new(r"^p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)$").unwrap();
    let robots = contents
        .lines()
        .map(|e| {
            let cap = re.captures(e).unwrap();
            Robot {
                position: (
                    cap["px"].parse::<i32>().unwrap(),
                    cap["py"].parse::<i32>().unwrap(),
                ),
                velocity: (
                    cap["vx"].parse::<i32>().unwrap(),
                    cap["vy"].parse::<i32>().unwrap(),
                ),
            }
        })
        .collect::<Vec<_>>();

    // for robot in robots {
    //     let new_x = (robot.position.0 + 100 * robot.velocity.0).rem_euclid(WIDTH);
    //     let new_y = (robot.position.1 + 100 * robot.velocity.1).rem_euclid(HEIGHT);
    //     map[new_y as usize][new_x as usize] += 1;
    // }
    // print_map(&map);
    //
    // let mut result = (0, 0, 0, 0);
    // let ranges = (
    //     (0..WIDTH / 2, (WIDTH / 2 + 1)..WIDTH),
    //     (0..HEIGHT / 2, (HEIGHT / 2 + 1)..HEIGHT),
    // );
    // for i in 0..HEIGHT {
    //     for j in 0..WIDTH {
    //         if ranges.0.0.contains(&j) && ranges.1.0.contains(&i) {
    //             result.0 += map[i as usize][j as usize];
    //         } else if ranges.0.1.contains(&j) && ranges.1.0.contains(&i) {
    //             result.1 += map[i as usize][j as usize];
    //         } else if ranges.0.0.contains(&j) && ranges.1.1.contains(&i) {
    //             result.2 += map[i as usize][j as usize];
    //         } else if ranges.0.1.contains(&j) && ranges.1.1.contains(&i) {
    //             result.3 += map[i as usize][j as usize];
    //         }
    //     }
    // }
    //
    // println!("{result:?}, {}", result.0 * result.1 * result.2 * result.3);

    let mut file = OpenOptions::new().append(true).open("output").unwrap();
    for i in 25000..35000 {
        for robot in robots.iter() {
            let new_x = (robot.position.0 + i * robot.velocity.0).rem_euclid(WIDTH);
            let new_y = (robot.position.1 + i * robot.velocity.1).rem_euclid(HEIGHT);
            map[new_y as usize][new_x as usize] += 1;
        }
        if let Err(e) = writeln!(file, "{i}") {
            eprintln!("Couldn't write to file: {}", e);
        }
        print_map(&mut file, &map);
        map = [[0; WIDTH as usize]; HEIGHT as usize];
        println!("{i} done");
    }
}
