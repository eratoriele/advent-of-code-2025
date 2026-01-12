use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Point = (usize, usize);

fn move_on_map(p: Point, map: &[Vec<u32>], visited_endpoints: &mut HashSet<Point>) {
    if map[p.0][p.1] == 9 {
        visited_endpoints.insert(p);
    } else {
        let next_points = [
            p.0.checked_sub(1).map(|x| (x, p.1)),
            p.0.checked_add(1).map(|x| (x, p.1)),
            p.1.checked_sub(1).map(|y| (p.0, y)),
            p.1.checked_add(1).map(|y| (p.0, y)),
        ];
        for &np in next_points.iter().flatten() {
            if (0..map.len()).contains(&np.1)
                && (0..map[0].len()).contains(&np.0)
                && map[np.0][np.1] == map[p.0][p.1] + 1
            {
                move_on_map(np, map, visited_endpoints);
            }
        }
    }
}

fn move_on_map_part2(p: Point, map: &[Vec<u32>], visited_endpoints: &mut HashMap<Point, usize>) {
    if map[p.0][p.1] == 9 {
        visited_endpoints
            .entry(p)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    } else {
        let next_points = [
            p.0.checked_sub(1).map(|x| (x, p.1)),
            p.0.checked_add(1).map(|x| (x, p.1)),
            p.1.checked_sub(1).map(|y| (p.0, y)),
            p.1.checked_add(1).map(|y| (p.0, y)),
        ];
        for &np in next_points.iter().flatten() {
            if (0..map.len()).contains(&np.1)
                && (0..map[0].len()).contains(&np.0)
                && map[np.0][np.1] == map[p.0][p.1] + 1
            {
                move_on_map_part2(np, map, visited_endpoints);
            }
        }
    }
}

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();
    let map: Vec<Vec<u32>> = contents
        .lines()
        .map(|e| {
            e.chars()
                .map(|level| level.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let starting_points: HashSet<Point> = map
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(
                move |(y, &level)| {
                    if level == 0 { Some((x, y)) } else { None }
                },
            )
        })
        .collect();

    let result = starting_points.iter().fold(0, |acc, &p| {
        let mut visited_endpoints = HashSet::new();
        move_on_map(p, &map, &mut visited_endpoints);
        acc + visited_endpoints.len()
    });
    println!("part1: {result}");
    let result = starting_points.iter().fold(0, |acc, &p| {
        let mut visited_endpoints = HashMap::new();
        move_on_map_part2(p, &map, &mut visited_endpoints);
        acc + visited_endpoints
            .iter()
            .fold(0, |acc_inner, endpoint| acc_inner + endpoint.1)
    });
    println!("part2: {result}");
}
