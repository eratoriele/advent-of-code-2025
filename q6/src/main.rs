use std::fs;

enum Operation {
    Add,
    Multiply,
}

fn do_opearation(numbers: &[u64], operator: &Operation) -> u64 {
    match operator {
        Operation::Add => numbers.iter().sum(),
        Operation::Multiply => numbers.iter().product(),
    }
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.trim();

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<Operation> = Vec::new();
    // get lines except the last line
    for line in contents
        .lines()
        .enumerate()
        .filter(|(i, _)| *i != contents.lines().count() - 1)
        .map(|(_, v)| v)
    {
        let mut line_vec: Vec<u64> = Vec::new();
        let elements = line.split_whitespace();
        for e in elements {
            line_vec.push(e.parse::<u64>().unwrap());
        }
        numbers.push(line_vec);
    }
    for ops in contents.lines().skip(contents.lines().count() - 1) {
        let split_ops = ops.split_whitespace();
        for o in split_ops {
            let operation = match o {
                "*" => Operation::Multiply,
                _ => Operation::Add,
            };
            operators.push(operation);
        }
    }

    let mut results: Vec<u64> = Vec::new();
    for i in 0..(operators.len()) {
        let mut nums: Vec<u64> = Vec::new();
        for n in numbers.iter() {
            nums.push(*n.get(i).unwrap());
        }
        results.push(do_opearation(&nums, operators.get(i).unwrap()));
    }

    let result: u64 = results.iter().sum();
    println!("{result}");
}
