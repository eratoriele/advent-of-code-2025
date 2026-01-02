use std::fs;

fn divide_string_to_arr(given_string: &str, size: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let iter = given_string.chars();
    for i in 0..(given_string.chars().count() / size) {
        let cloned_iter = iter.clone();
        result.push(cloned_iter.skip(i * size).take(size).collect());
    }
    result
}

fn main() {
    // let file_path = "./input/example.txt";
    let file_path = "./input/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut invalid_ids: Vec<u64> = Vec::new();

    for range in contents.split(',') {
        let mut iter = range.split('-');
        let start = iter.next().unwrap().trim().parse::<u64>().unwrap();
        let end = iter.next().unwrap().trim().parse::<u64>().unwrap();

        for i in start..=end {
            let string_i = i.to_string();
            let string_length = string_i.chars().count();

            // part 1
            // if string_length % 2 != 0 {
            //     continue;
            // }
            // let first_half: String = string_i.chars().take(string_length / 2).collect();
            // let second_half: String = string_i
            //     .chars()
            //     .skip(string_length / 2)
            //     .take(string_length / 2)
            //     .collect();
            //
            // if first_half == second_half {
            //     invalid_ids.push(i);
            // }

            // part 2
            for char_index in 1..=string_i.chars().count() {
                if (char_index) > string_length / 2 {
                    break;
                }
                if string_length % (char_index) != 0 {
                    continue;
                }

                let pattern_check: String = string_i.chars().take(char_index).collect();
                let remaining_string: String = string_i.chars().skip(char_index).collect();
                let remaining_strings_of_equal_length =
                    divide_string_to_arr(&remaining_string, char_index);

                let mut success = true;
                for s in remaining_strings_of_equal_length {
                    if pattern_check != s {
                        success = false;
                        break;
                    }
                }

                if success {
                    // println!("{pattern_check}, {i}");
                    invalid_ids.push(i);
                }
            }
        }
    }

    invalid_ids.sort();
    invalid_ids.dedup();
    println!("{:?}", invalid_ids);
    let sum: u64 = invalid_ids.iter().sum();
    println!("{sum}");
}
