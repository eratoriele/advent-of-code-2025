use std::fs;

#[derive(Clone)]
enum PointType {
    Start,
    Beam,
    Splitter,
    Space,
}
impl std::fmt::Debug for PointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match self {
            PointType::Start => &"Start",
            PointType::Beam => &"Beam",
            PointType::Splitter => &"Splitter",
            PointType::Space => &"Space",
        };
        f.debug_struct("PointType")
            .field("type", printable)
            .finish()
    }
}
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }
}

fn print_line(line: &[PointType]) {
    for point in line {
        let val = match point {
            PointType::Space => ".",
            PointType::Start => "S",
            PointType::Splitter => "^",
            PointType::Beam => "|",
        };
        print!("{val}");
    }
    println!();
}

fn move_beam_down(map: &[Vec<PointType>], level: usize) -> (Vec<Vec<PointType>>, u64) {
    let mut return_map = map.to_vec();
    let mut split_count = 0u64;

    for (index, point) in return_map[level].clone().iter().enumerate() {
        if let PointType::Beam = point {
            if let PointType::Splitter = return_map[level + 1][index] {
                split_count += 1;
                return_map[level + 1][index - 1] = PointType::Beam;
                return_map[level + 1][index + 1] = PointType::Beam;
            } else if let PointType::Space = return_map[level + 1][index] {
                return_map[level + 1][index] = PointType::Beam;
            }
        }
    }
    print_line(&return_map[level + 1]);

    (return_map, split_count)
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut map: Vec<Vec<PointType>> = Vec::new();
    let mut start_point = Point::default();
    for (line_number, line) in contents.lines().enumerate() {
        map.push(Vec::new());
        for (col_number, char) in line.chars().enumerate() {
            map[line_number].push(match char {
                'S' => {
                    start_point.y = col_number;
                    PointType::Start
                }
                '^' => PointType::Splitter,
                _ => PointType::Space,
            });
        }
    }
    let start_point = start_point; // mut the value
    map[start_point.x + 1][start_point.y] = PointType::Beam; // the first beam

    let mut total_splits = 0u64;
    // The last line is empty
    for i in 1..(contents.lines().count() - 1) {
        let (new_map, splits) = move_beam_down(&map, i);
        map = new_map;
        total_splits += splits;
    }

    println!("{total_splits}");
}
