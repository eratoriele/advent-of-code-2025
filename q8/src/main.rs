use std::fs;

const CONNECTION_COUNT: usize = 1000;

#[derive(Debug)]
struct Point3D(u64, u64, u64);

#[derive(Clone, Copy, Debug)]
struct Connection {
    distance: f64,
    point_indexes: (u16, u16),
}
impl Connection {
    fn default() -> Connection {
        Connection {
            distance: f64::MAX,
            point_indexes: (0, 0),
        }
    }
}

#[derive(Debug)]
struct ConnectionStructure {
    indexes: Vec<u16>,
}

fn calculate_distance(first: &Point3D, second: &Point3D) -> f64 {
    let distance: f64 = (first.0.abs_diff(second.0).pow(2)
        + first.1.abs_diff(second.1).pow(2)
        + first.2.abs_diff(second.2).pow(2)) as f64;

    distance.sqrt()
}

fn should_replace(arr: &[Connection], distance: f64) -> (bool, u16) {
    let mut replace = false;
    let mut index = CONNECTION_COUNT as u16;
    for conn in arr.iter().rev() {
        if distance > conn.distance {
            break;
        }
        replace = true;
        index -= 1;
    }
    (replace, index)
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut points: Vec<Point3D> = Vec::new();
    for line in contents.lines() {
        let mut numbers = line.split(',');
        let point = Point3D(
            numbers.next().unwrap().parse::<u64>().unwrap(),
            numbers.next().unwrap().parse::<u64>().unwrap(),
            numbers.next().unwrap().parse::<u64>().unwrap(),
        );
        points.push(point);
    }

    let mut smallest_distances: Vec<Connection> = vec![Connection::default(); CONNECTION_COUNT];
    for (first_index, first) in points.iter().enumerate() {
        for (second_index, second) in points.iter().skip(first_index + 1).enumerate() {
            let distance = calculate_distance(first, second);
            let (replace, index) = should_replace(&smallest_distances, distance);
            if replace {
                smallest_distances.insert(
                    index as usize,
                    Connection {
                        distance,
                        point_indexes: (
                            first_index as u16,
                            (second_index + first_index + 1) as u16,
                        ),
                    },
                );
                smallest_distances.pop();
            }
        }
    }

    let mut structures: Vec<ConnectionStructure> = Vec::new();
    for conn in smallest_distances.iter() {
        let zero_included = structures
            .iter()
            .position(|e| e.indexes.contains(&conn.point_indexes.0));
        let one_included = structures
            .iter()
            .position(|e| e.indexes.contains(&conn.point_indexes.1));

        // if both indexes are already part of connections
        if let Some(i_0) = zero_included
            && let Some(i_1) = one_included
        {
            // the they alread are in the same connection
            if i_0 == i_1 {
                continue;
            }
            let second_struct = structures[i_1].indexes.clone();
            structures[i_0].indexes.extend(second_struct);
            structures.remove(i_1);
        } else if let Some(i) = zero_included {
            // if only first one is in a connection
            structures[i].indexes.push(conn.point_indexes.1);
        } else if let Some(i) = one_included {
            // if only second one is in a connection
            structures[i].indexes.push(conn.point_indexes.0);
        } else {
            // if neither is in a connection
            structures.push(ConnectionStructure {
                indexes: vec![conn.point_indexes.0, conn.point_indexes.1],
            });
        }
    }

    structures.sort_by(|a, b| b.indexes.len().cmp(&a.indexes.len()));
    println!("{:#?}", structures);
    println!(
        "{}",
        structures
            .iter()
            .take(3)
            .fold(1, |acc, x| acc * x.indexes.len())
    );
}
