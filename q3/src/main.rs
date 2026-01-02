use std::fs;

fn get_biggest_digit(list: &[u8]) -> (u8, usize) {
    let mut highest = 0u8;
    let mut highest_index = 0;
    for i in 0..list.len() {
        if highest == 9 {
            break;
        } else if list[i] > highest {
            highest = list[i];
            highest_index = i;
        }
    }
    (highest, highest_index)
}

fn main() {
    // let file_path = "./input/example.txt";
    let file_path = "./input/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut joltages: Vec<u64> = Vec::new();
    let joltage_len = 12;

    for line in contents.lines() {
        let mut joltage: Vec<u8> = Vec::new();
        let line_vec: Vec<u8> = line
            .chars()
            .map(|e| e.to_string().parse::<u8>().unwrap())
            .collect();

        // part 1
        // let (highest, highest_index) = get_biggest_digit(&line_vec[0..(line_vec.len() - 1)]);
        // let (second_highest, _) = get_biggest_digit(&line_vec[(highest_index + 1)..line_vec.len()]);
        //
        // let joltage = (highest * 10 + second_highest) as u64;
        // joltages.push(joltage);

        // part 2
        let (highest, mut highest_index) =
            get_biggest_digit(&line_vec[0..(line_vec.len() - joltage_len + 1)]);
        joltage.push(highest);
        for i in 1..joltage_len {
            let (highest, highest_index_next) = get_biggest_digit(
                &line_vec[(highest_index + 1)..=(line_vec.len() - joltage_len + i)],
            );
            joltage.push(highest);
            highest_index += highest_index_next + 1;
        }

        let final_joltage: String = joltage.iter().map(|e| e.to_string()).collect();
        let final_joltage = final_joltage.parse::<u64>().unwrap();
        joltages.push(final_joltage);
    }

    let sum: u64 = joltages.iter().sum();
    println!("{sum}");
}
