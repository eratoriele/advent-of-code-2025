// use std::env;
use std::fs;

fn main() {
    let file_path = "./input/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("Input: \n{contents}");

    let mut dial = 50;
    let mut zeroes = 0;
    let mut prev_zero = false;

    for line in contents.lines() {
        let mut iter = line.chars();
        let direction = iter.next().unwrap();
        let amount = iter.as_str().parse::<i32>().unwrap();

        match direction {
            'L' => dial -= amount,
            'R' => dial += amount,
            _ => (),
        };

        // part 1:
        // dial %= 100;
        // if dial == 0 {
        //     zeroes += 1;
        // }
        //part 2:
        zeroes += (dial / 100).abs();
        if !prev_zero && dial <= 0 {
            zeroes += 1;
        }
        println!("dial: {}, zeroes: {}", dial, zeroes);
        dial = dial.rem_euclid(100);
        prev_zero = dial == 0;
    }

    println!("answer: {}", zeroes);
}
