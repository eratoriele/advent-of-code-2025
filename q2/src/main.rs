use std::fs;

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
        }
    }

    let sum: u64 = invalid_ids.iter().sum();
    println!("{sum}");
}
