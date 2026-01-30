use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

enum Node {
    Wall,
    Start,
    End,
    Space,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position(usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    cost: u64,
    pos: Position,
    dir: (i8, i8),
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

    let mut start_pos = Position(0, 0);
    let mut end_pos = Position(0, 0);
    let map = contents
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let row = line
                .chars()
                .map(|c| match c {
                    '.' => Node::Space,
                    'S' => Node::Start,
                    'E' => Node::End,
                    _ => Node::Wall,
                })
                .collect::<Vec<_>>();
            if let Some(j) = row.iter().position(|e| matches!(e, Node::Start)) {
                start_pos = Position(i, j);
            }
            if let Some(j) = row.iter().position(|e| matches!(e, Node::End)) {
                end_pos = Position(i, j);
            }
            row
        })
        .collect::<Vec<_>>();

    let mut dists = HashMap::new();
    let mut predecessors: HashMap<(Position, (i8, i8)), Vec<(Position, (i8, i8))>> = HashMap::new();
    let mut pq = BinaryHeap::new();

    pq.push(State {
        cost: 0,
        pos: start_pos,
        dir: (0, 1),
    });
    dists.insert((start_pos, (0, 1)), 0);

    let mut end_cost = u64::MAX;

    while let Some(State { cost, pos, dir }) = pq.pop() {
        if pos == end_pos {
            if cost < end_cost {
                end_cost = cost;
            }
            continue;
        }
        if cost > *dists.get(&(pos, dir)).unwrap_or(&u64::MAX) {
            continue;
        }
        let moves = [
            (dir, 1),                // Forward
            ((-dir.1, dir.0), 1001), // Turn Left + Step
            ((dir.1, -dir.0), 1001), // Turn Right + Step
        ];

        for (next_dir, move_cost) in moves {
            let next_pos = Position(
                pos.0.saturating_add_signed(next_dir.0 as isize),
                pos.1.saturating_add_signed(next_dir.1 as isize),
            );

            if let Node::Wall = map[next_pos.0][next_pos.1] {
                continue;
            }

            let next_cost = cost + move_cost;
            let current_best = *dists.get(&(next_pos, next_dir)).unwrap_or(&u64::MAX);

            if next_cost < current_best {
                dists.insert((next_pos, next_dir), next_cost);
                predecessors.insert((next_pos, next_dir), vec![(pos, dir)]);
                pq.push(State {
                    cost: next_cost,
                    pos: next_pos,
                    dir: next_dir,
                });
            } else if next_cost == current_best {
                predecessors
                    .entry((next_pos, next_dir))
                    .or_default()
                    .push((pos, dir));
            }
        }
    }
    let mut best_path_tiles = HashSet::new();
    let mut queue = VecDeque::new();

    for &d in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if *dists.get(&(end_pos, d)).unwrap_or(&u64::MAX) == end_cost {
            queue.push_back((end_pos, d));
        }
    }

    let mut seen_states = HashSet::new();
    while let Some(state) = queue.pop_front() {
        if !seen_states.insert(state) {
            continue;
        }
        best_path_tiles.insert(state.0);

        if let Some(parents) = predecessors.get(&state) {
            for &parent in parents {
                queue.push_back(parent);
            }
        }
    }

    println!("{}", best_path_tiles.len());
}
