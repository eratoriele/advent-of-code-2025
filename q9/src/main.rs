use std::fs;

#[derive(Debug)]
struct Point(u64, u64);

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut points: Vec<Point> = Vec::new();
    for line in contents.lines() {
        let mut split = line.split(',');
        points.push(Point(
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    // part 1
    {
        let mut largest_area = 0u64;
        for (first_index, first) in points.iter().enumerate() {
            for second in points.iter().skip(first_index + 1) {
                let area = (first.0.abs_diff(second.0) + 1) * (first.1.abs_diff(second.1) + 1);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
        println!("{largest_area}");
    }

    //part 2
    // gave up, looked it up online...
    {
        let edges: Vec<(&Point, &Point)> = points
            .windows(2)
            .map(|vertices| (&vertices[0], &vertices[1]))
            .chain([(&points[points.len() - 1], &points[0])]) // wrap the end
            .collect();
        let mut possible_rects: Vec<(&Point, &Point, u64)> = points
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| {
                points[(i + 1)..].iter().map(move |p2| {
                    (
                        p1,
                        p2,
                        (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1),
                    )
                })
            })
            .collect();
        possible_rects.sort_by(|p1, p2| p2.2.cmp(&p1.2));
        let result = possible_rects
            .into_iter()
            .find(|(p1, p2, _)| {
                edges.iter().all(|(start, end)| {
                    let before = p1.1.max(p2.1) <= start.1.min(end.1);
                    let after = p1.1.min(p2.1) >= start.1.max(end.1);
                    let above = p1.0.max(p2.0) <= start.0.min(end.0);
                    let below = p1.0.min(p2.0) >= start.0.max(end.0);
                    before || after || above || below
                })
            })
            .unwrap()
            .2;

        println!("{result}");
    }
}
