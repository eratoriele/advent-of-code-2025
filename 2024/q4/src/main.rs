use std::fs;

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();
    let input = contents
        .lines()
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    let search_word = ['X', 'M', 'A', 'S'];
    let directions = [
        (0, 1),   // right
        (0, -1),  //left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down right
        (1, -1),  // down left
        (-1, -1), // up left
        (-1, 1),  // up right
    ];
    let mut result = 0u32;
    for i in 0..input[0].len() {
        for j in 0..input.len() {
            if input[i][j] != search_word[0] {
                continue;
            }
            for (x, y) in directions {
                let mut found = true;

                for (k, &ch) in search_word.iter().skip(1).enumerate() {
                    let nr = i as i32 + x * ((k as i32) + 1);
                    let nc = j as i32 + y * ((k as i32) + 1);

                    // Check bounds
                    if nr < 0 || nr >= input[0].len() as i32 || nc < 0 || nc >= input.len() as i32 {
                        found = false;
                        break;
                    }

                    // Check character match
                    if input[nr as usize][nc as usize] != ch {
                        found = false;
                        break;
                    }
                }

                if found {
                    result += 1;
                }
            }
        }
    }
    println!("{result}");

    //part 2
    let cross_chars = [('S', 'M'), ('M', 'S')];
    result = 0;
    for i in 1..(input[0].len() - 1) {
        for j in 1..(input.len() - 1) {
            if input[i][j] != 'A' {
                continue;
            }
            let cross_one = (input[i - 1][j - 1], input[i + 1][j + 1]);
            let cross_two = (input[i - 1][j + 1], input[i + 1][j - 1]);
            if cross_chars.contains(&cross_one) && cross_chars.contains(&cross_two) {
                result += 1;
            }
        }
    }
    println!("{result}");
}
