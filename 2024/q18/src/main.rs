use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

const MAP_SIZE: usize = 71;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position(usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    cost: u64,
    pos: Position,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // let input_file = "input/example";
    let input_file = "input/input";

    let contents = fs::read_to_string(input_file).expect("Input file expected");
    let mut map = [[0; MAP_SIZE]; MAP_SIZE];
    for (i, line) in contents.lines().enumerate() {
        let coords = line
            .split(',')
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        map[coords[1]][coords[0]] = 1;
        let start_position = Position(0, 0);

        let mut dists = HashMap::new();
        let mut pq = BinaryHeap::new();

        pq.push(State {
            cost: 0,
            pos: start_position,
        });
        dists.insert(start_position, 0);

        let mut goal_reached = false;
        while let Some(State { cost, pos }) = pq.pop() {
            if pos.0 == MAP_SIZE - 1 && pos.1 == MAP_SIZE - 1 {
                println!("{i}: {}", cost);
                goal_reached = true;
                break;
            }
            if cost > *dists.get(&pos).unwrap_or(&u64::MAX) {
                continue;
            }

            let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            for m in moves {
                let next_pos = Position(
                    pos.0.saturating_add_signed(m.0 as isize),
                    pos.1.saturating_add_signed(m.1 as isize),
                );
                if next_pos.0 < MAP_SIZE
                    && next_pos.1 < MAP_SIZE
                    && map[next_pos.0][next_pos.1] != 1
                {
                    let next_cost = cost + 1;
                    if next_cost < *dists.get(&next_pos).unwrap_or(&u64::MAX) {
                        dists.insert(next_pos, next_cost);
                        pq.push(State {
                            cost: next_cost,
                            pos: next_pos,
                        });
                    }
                }
            }
        }
        if !goal_reached {
            println!("No paths remain {i}, {line}");
            break;
        }
    }
}
