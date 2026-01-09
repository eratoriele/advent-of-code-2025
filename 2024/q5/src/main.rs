use std::fs;

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    let mut rules: Vec<(u8, u8)> = Vec::new();
    let mut manuals: Vec<Vec<u8>> = Vec::new();

    let mut get_rules = true;
    for line in contents.lines() {
        if line.is_empty() {
            get_rules = false;
            continue;
        }

        if get_rules {
            let mut pages = line.split('|');
            rules.push((
                pages.next().unwrap().parse::<u8>().unwrap(),
                pages.next().unwrap().parse::<u8>().unwrap(),
            ));
        } else {
            manuals.push(
                line.split(',')
                    .map(|e| e.parse::<u8>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }

    // println!("{rules:?}, {manuals:?}");

    let mut result = 0u64;
    for manual in manuals {
        let mut mutable_manual = manual.to_vec();
        let mut legal = true;
        let mut retry = true;
        while retry {
            retry = false;
            for (x, y) in rules.as_slice() {
                let Some(x_pos) = mutable_manual.iter().position(|e| e == x) else {
                    continue;
                };
                let Some(y_pos) = mutable_manual.iter().position(|e| e == y) else {
                    continue;
                };

                if x_pos > y_pos {
                    legal = false;
                    mutable_manual[x_pos] = *y;
                    mutable_manual[y_pos] = *x;
                    retry = true;
                    break;
                }
            }
        }
        if !legal {
            result += mutable_manual[mutable_manual.len() / 2] as u64;
        }
    }

    println!("{result}");
}
