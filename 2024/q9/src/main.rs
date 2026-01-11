use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum File {
    Filled(u32),
    Empty,
}

#[derive(Debug, Clone, Copy)]
enum FileDefrag {
    Filled(u32, u32),
    Empty(u32),
}

fn find_swap_positions(file_system: &[File]) -> (usize, usize) {
    let mut ret_data = (0, 0);
    for (index, f) in file_system.iter().enumerate() {
        if *f == File::Empty {
            ret_data.0 = index;
            break;
        }
    }
    for (index, f) in file_system.iter().enumerate().rev() {
        if *f != File::Empty {
            ret_data.1 = index;
            break;
        }
    }

    ret_data
}

fn part1(contents: &str) {
    let mut file_system: Vec<File> = Vec::new();

    for (index, ch) in contents.trim().chars().enumerate() {
        if index % 2 == 0 {
            file_system.extend(vec![
                File::Filled((index as u32) / 2);
                ch.to_string().parse::<usize>().unwrap()
            ]);
        } else {
            file_system.extend(vec![File::Empty; ch.to_string().parse::<usize>().unwrap()]);
        }
    }

    loop {
        let (first, second) = find_swap_positions(&file_system);
        if first > second {
            break;
        } else {
            file_system[first] = file_system[second];
            file_system[second] = File::Empty;
        }
    }

    println!(
        "{}",
        file_system.iter().enumerate().fold(0, |acc, (index, &e)| {
            match e {
                File::Filled(id) => acc + (id as usize) * index,
                File::Empty => acc,
            }
        })
    )
}

fn find_first_space(fs: &[FileDefrag], min_size: u32, max_index: usize) -> (bool, usize) {
    let mut result = (false, 0);
    for (i, &f) in fs.iter().enumerate() {
        if i >= max_index {
            break;
        }
        if let FileDefrag::Empty(size) = f
            && size >= min_size
        {
            result = (true, i);
            break;
        }
    }
    result
}
fn combine_empty_spaces(file_system: Vec<FileDefrag>) -> Vec<FileDefrag> {
    let mut combined_fs: Vec<FileDefrag> = Vec::new();

    // Iterate through the original vector
    for defrag_item in file_system.into_iter() {
        match defrag_item {
            FileDefrag::Filled(_, _) => {
                combined_fs.push(defrag_item);
            }
            FileDefrag::Empty(new_size) => {
                if let Some(FileDefrag::Empty(current_size)) = combined_fs.last_mut() {
                    *current_size += new_size;
                } else {
                    combined_fs.push(FileDefrag::Empty(new_size));
                }
            }
        }
    }

    combined_fs
}

fn part2(contents: &str) {
    let mut file_system: Vec<FileDefrag> = Vec::new();

    for (index, ch) in contents.trim().chars().enumerate() {
        let count = ch.to_string().parse::<u32>().unwrap();
        if index % 2 == 0 {
            file_system.push(FileDefrag::Filled(index as u32 / 2, count));
        } else if count > 0 {
            file_system.push(FileDefrag::Empty(count));
        }
    }

    let mut defragged = file_system.to_vec();
    // println!("{defragged:?}");
    for &f in file_system.iter().rev() {
        if let FileDefrag::Filled(id, length) = f {
            let defragged_index = defragged
                .iter()
                .position(|&e| {
                    if let FileDefrag::Filled(id_defrag, length_defrag) = e {
                        id_defrag == id && length == length_defrag
                    } else {
                        false
                    }
                })
                .unwrap();
            let (movable, move_to) = find_first_space(&defragged, length, defragged_index);
            // println!("{defragged_index}, {id}, {movable}, {move_to}");
            if movable && let FileDefrag::Empty(removed_space) = defragged.remove(move_to) {
                defragged[defragged_index - 1] = FileDefrag::Empty(length);
                defragged.insert(move_to, f);
                if removed_space > length {
                    defragged.insert(move_to + 1, FileDefrag::Empty(removed_space - length));
                }
                defragged = combine_empty_spaces(defragged);

                // println!("{defragged:?}");
            }
        }
    }

    println!("{defragged:?}");
    let mut fs: Vec<File> = Vec::new();
    for &f in defragged.iter() {
        match f {
            FileDefrag::Filled(id, length) => fs.extend(vec![File::Filled(id); length as usize]),
            FileDefrag::Empty(length) => fs.extend(vec![File::Empty; length as usize]),
        }
    }
    println!("{fs:?}");
    for &f in fs.iter() {
        print!(
            "{},",
            match f {
                File::Filled(id) => id.to_string(),
                File::Empty => ".".to_string(),
            }
        );
    }
    println!();
    let mut result = 0usize;
    for (i, &f) in fs.iter().enumerate() {
        if let File::Filled(id) = f {
            result += id as usize * i;
        }
    }
    println!("{result}");
}

fn main() {
    // let file_name = "input/example";
    let file_name = "input/input";

    let contents = fs::read_to_string(file_name).unwrap();

    // part1(&contents);
    part2(&contents);
}
