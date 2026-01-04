use std::fs;

const LINE_COUNT: usize = 5;

struct Equation {
    values: [Vec<char>; LINE_COUNT - 1],
    op: Operation,
}
impl std::fmt::Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Equation")
            .field("values", &self.values)
            .field("op", &self.op)
            .finish()
    }
}
impl Equation {
    fn default() -> Self {
        Equation {
            values: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            op: Operation::Add,
        }
    }
}

enum Operation {
    Add,
    Multiply,
}
impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match self {
            Operation::Add => &"add",
            Operation::Multiply => &"mul",
        };
        f.debug_struct("Operation").field("op", printable).finish()
    }
}

fn bug_math(eq: &Equation) -> u64 {
    let num_count = eq.values[0].len();
    let mut numbers: Vec<String> = vec![String::from(""); num_count];
    for i in 0..(LINE_COUNT - 1) {
        for j in 0..num_count {
            numbers[j] = numbers[j].clone() + &eq.values[i][j].to_string();
        }
    }
    let mut parsed_numbers: Vec<u64> = Vec::new();
    for n in numbers {
        let trimmed = n.trim();
        if !trimmed.is_empty() {
            let parsed = trimmed.parse::<u64>().unwrap();
            parsed_numbers.push(parsed);
        }
    }
    match eq.op {
        Operation::Add => parsed_numbers.iter().sum(),
        Operation::Multiply => parsed_numbers.iter().product(),
    }
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut equations: Vec<Equation> = Vec::new();
    let lines: Vec<&str> = contents.lines().collect();

    let mut cur_eq: Option<Equation> = None;
    let mut seq_ended = [true; LINE_COUNT];
    for col in 0..(lines.first().unwrap().len()) {
        if cur_eq.is_none() {
            cur_eq = Some(Equation::default());
        }
        for (line_number, line) in (lines).iter().enumerate() {
            let ch = line.as_bytes()[col] as char;
            seq_ended[line_number] = ch == ' ';

            // Last line has the equation
            if line_number == LINE_COUNT - 1 {
                if (ch == '*' || ch == '+')
                    && let Some(mut eq) = cur_eq.take()
                {
                    eq.op = match ch {
                        '*' => Operation::Multiply,
                        _ => Operation::Add,
                    };
                    cur_eq = Some(eq);
                }
            } else if let Some(mut eq) = cur_eq.take() {
                eq.values[line_number].push(ch);
                cur_eq = Some(eq);
            }
        }
        if seq_ended.iter().all(|e| *e) || col == lines.first().unwrap().len() - 1 {
            if let Some(eq) = cur_eq.take() {
                equations.push(eq);
            }
            cur_eq = None;
        }
        seq_ended = [false; LINE_COUNT];
    }

    let mut results: Vec<u64> = Vec::new();
    for e in equations {
        results.push(bug_math(&e));
    }
    let final_sum: u64 = results.iter().sum();
    println!("{final_sum}");
}
