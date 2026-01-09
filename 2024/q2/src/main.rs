use std::fs;

const MIN_CHANGE: u8 = 1;
const MAX_CHANGE: u8 = 3;

fn check_levels(numbers: &[u8]) -> (bool, usize) {
    let mut fail_level = numbers.len() + 1;
    let increasing = numbers[0] < numbers[1];
    for (index, pair) in numbers.windows(2).enumerate() {
        if increasing != (pair[0] < pair[1]) {
            fail_level = index;
            break;
        }
        if !(MIN_CHANGE..=MAX_CHANGE).contains(&pair[0].abs_diff(pair[1])) {
            fail_level = index;
            break;
        }
    }
    (fail_level > numbers.len(), fail_level)
}

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();
    let mut answer = 0u32;
    for line in contents.lines() {
        let numbers = line
            .split_whitespace()
            .map(|e| e.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        //part 1
        {
            // let increasing = numbers[0] < numbers[1];
            // if numbers.windows(2).into_iter().all(|e| {
            //     increasing == (e[0] < e[1])
            //         && (MIN_CHANGE..=MAX_CHANGE).contains(&e[0].abs_diff(e[1]))
            // }) {
            //     answer += 1;
            // }
        }

        //part 2
        {
            let (success, fail_level) = check_levels(numbers.as_slice());
            if success {
                answer += 1;
            } else {
                let mut first_removed = numbers.to_vec();
                first_removed.remove(fail_level);
                let (success, _) = check_levels(first_removed.as_slice());
                if success {
                    answer += 1;
                } else {
                    let mut second_removed = numbers.to_vec();
                    second_removed.remove(fail_level + 1);
                    let (success, _) = check_levels(second_removed.as_slice());
                    if success {
                        answer += 1;
                    } else if fail_level > 0 {
                        let mut third_removed = numbers.to_vec();
                        third_removed.remove(fail_level - 1);
                        let (success, _) = check_levels(third_removed.as_slice());
                        if success {
                            answer += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{answer}");
}
