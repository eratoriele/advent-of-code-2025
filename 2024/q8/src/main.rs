use std::{
    collections::{HashMap, HashSet},
    fs, result,
};

fn create_combinations(vec: Vec<(i8, i8)>) -> HashSet<((i8, i8), (i8, i8))> {
    let mut ret = HashSet::new();
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            ret.insert((vec[i], vec[j]));
        }
    }
    ret
}

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut uniqs: HashMap<char, Vec<(i8, i8)>> = HashMap::new();
    let mut antizones: HashSet<(i8, i8)> = HashSet::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                uniqs.entry(c).or_default().push((i as i8, j as i8));
                antizones.insert((i as i8, j as i8));
            }
        }
    }

    let x_range = 0..map[0].len() as i8;
    let y_range = 0..map.len() as i8;

    for (_, pos) in uniqs {
        for (pos1, pos2) in create_combinations(pos) {
            let mut counter = 0i8;
            let mut points_added;
            loop {
                counter += 1;
                points_added = (false, false);
                let distance = (pos1.0 - pos2.0, pos1.1 - pos2.1);
                let points = (
                    (pos1.0 + distance.0 * counter, pos1.1 + distance.1 * counter),
                    (pos2.0 - distance.0 * counter, pos2.1 - distance.1 * counter),
                );
                if x_range.contains(&points.0.0) && y_range.contains(&points.0.1) {
                    antizones.insert(points.0);
                    points_added.0 = true;
                }
                if x_range.contains(&points.1.0) && y_range.contains(&points.1.1) {
                    antizones.insert(points.1);
                    points_added.1 = true;
                }
                if !points_added.0 && !points_added.1 {
                    break;
                }
            }
        }
    }
    println!("{antizones:?}, {}", antizones.len());
}
