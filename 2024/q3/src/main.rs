use regex::Regex;
use std::fs;

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    // part 1
    let mut result = 0u64;
    let mut do_mul = true;
    let re = Regex::new(r"mul\((?<first>[0-9]+),(?<second>[0-9]+)\)|do\(\)|don't\(\)").unwrap();
    for line in contents.lines() {
        println!("{line}");
        let caps = re.captures_iter(line);
        for c in caps {
            let captured = c.get_match().as_str();
            println!("{captured}");
            // println!("{} * {}", &c["first"], &c["second"]);
            match captured {
                "do()" => do_mul = true,
                "don't()" => do_mul = false,
                _ => {
                    if do_mul {
                        result +=
                            c["first"].parse::<u64>().unwrap() * c["second"].parse::<u64>().unwrap()
                    }
                }
            }
            // result += c["first"].parse::<u64>().unwrap() * c["second"].parse::<u64>().unwrap();
        }
    }

    println!("{result}");
}
