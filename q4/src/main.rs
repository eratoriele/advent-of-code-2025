use std::{fs, ops::RangeInclusive};

fn get_adjacents(x: u16, constraint: u16) -> RangeInclusive<u16> {
    let min = x.saturating_sub(1);
    let max = x.saturating_add(1).min(constraint);

    min..=max
}

fn paper_cleanable(x: u16, y: u16, array: &[Vec<char>], width: u16, height: u16) -> bool {
    let mut paper_around = 0u8;
    let mut return_value = true;
    'outer: for i in get_adjacents(x, width) {
        for j in get_adjacents(y, height) {
            if i == x && j == y {
                continue;
            } else if array[i as usize][j as usize] == '@' {
                paper_around += 1;
                if paper_around >= 4 {
                    return_value = false;
                    break 'outer;
                }
            }
        }
    }
    return_value
}

fn main() {
    // let file_path = "./input/example.txt";
    let file_path = "./input/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.trim();
    let height = contents.lines().count();
    let width = contents.lines().next().unwrap().chars().count();

    let mut given_array: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let line_vec = line.chars().collect();
        given_array.push(line_vec);
    }

    let mut prev_result = 1u32; // start at 1 to enter the loop
    let mut result = 0u32;
    while prev_result != result {
        prev_result = result;
        for i in 0..width {
            for j in 0..height {
                if given_array[i][j] == '@'
                    && paper_cleanable(
                        i as u16,
                        j as u16,
                        &given_array,
                        (width - 1) as u16,
                        (height - 1) as u16,
                    )
                {
                    result += 1;
                    given_array[i][j] = '.';
                }
            }
        }
    }

    println!("{result}");
}
