use std::fs;

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let mut numbers: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
    for line in contents.lines() {
        let mut iter = line.split_whitespace().collect::<Vec<_>>().into_iter();
        numbers.0.push(iter.next().unwrap().parse::<u32>().unwrap());
        numbers.1.push(iter.next().unwrap().parse::<u32>().unwrap());
    }

    //part 1
    {
        // numbers.0.sort();
        // numbers.1.sort();
        //
        // let answer: u64 = numbers
        //     .0
        //     .iter()
        //     .zip(numbers.1)
        //     .fold(0, |acc, (a, b)| acc + a.abs_diff(b) as u64);
        //
        // println!("{answer}");
    }

    //part 2
    {
        let answer: u64 = numbers.0.iter().fold(0, |acc, a| {
            acc + (*a as u64 * numbers.1.iter().filter(|b| *b == a).count() as u64)
        });

        println!("{answer}");
    }
}
