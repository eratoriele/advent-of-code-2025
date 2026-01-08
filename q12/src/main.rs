use std::fs;

type Shape = [[bool; 9]; 8];
#[derive(Debug)]
struct Grid {
    width: u8,
    height: u8,
    requirements: Vec<u8>,
}

fn rotate_shape_90(shape: [bool; 9]) -> [bool; 9] {
    [
        shape[6], shape[3], shape[0], shape[7], shape[4], shape[1], shape[8], shape[5], shape[2],
    ]
}
fn flip_shape(shape: [bool; 9]) -> [bool; 9] {
    [
        shape[2], shape[1], shape[0], shape[5], shape[4], shape[3], shape[8], shape[7], shape[6],
    ]
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut shapes: Vec<Shape> = Vec::new();
    for shape_input in contents.lines().take(30).collect::<Vec<_>>().chunks(5) {
        let mut s = [false; 9];
        for (line_num, line) in shape_input[1..4].iter().enumerate() {
            let mut iter = line.chars();
            s[line_num * 3] = iter.next().unwrap() == '#';
            s[(line_num * 3) + 1] = iter.next().unwrap() == '#';
            s[(line_num * 3) + 2] = iter.next().unwrap() == '#';
        }
        shapes.push([
            s,
            flip_shape(s),
            rotate_shape_90(s),
            flip_shape(rotate_shape_90(s)),
            rotate_shape_90(rotate_shape_90(s)),
            flip_shape(rotate_shape_90(rotate_shape_90(s))),
            rotate_shape_90(rotate_shape_90(rotate_shape_90(s))),
            flip_shape(rotate_shape_90(rotate_shape_90(rotate_shape_90(s)))),
        ]);
    }

    let mut grids: Vec<Grid> = Vec::new();
    for line in contents.lines().skip(30) {
        let sections = line.split(": ").collect::<Vec<_>>();
        let size = sections[0].split('x').collect::<Vec<_>>();
        grids.push(Grid {
            width: size[0].parse().unwrap(),
            height: size[1].parse().unwrap(),
            requirements: sections[1]
                .split_whitespace()
                .map(|e| e.parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        });
    }

    // dont bother with shapes, just get the bounds
    let mut max_possible = 0u32;
    let mut min_possible = 0u32;
    for grid in grids {
        let available_space: u32 = grid.width as u32 * grid.height as u32;
        let total_required_space: u32 =
            grid.requirements.iter().enumerate().fold(0, |acc, (i, e)| {
                acc + shapes[i][0]
                    .iter()
                    .fold(0, |acc, sp| if *sp { acc + 1 } else { acc })
                    * (*e as u32)
            });
        let max_place_needed: u32 = grid
            .requirements
            .iter()
            .enumerate()
            .fold(0, |acc, (_, e)| acc + 9 * (*e as u32));
        if available_space > total_required_space {
            max_possible += 1;
        }
        if available_space > max_place_needed {
            min_possible += 1;
        }
    }
    println!("{max_possible} {min_possible}");
}
