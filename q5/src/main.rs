use std::fs;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}
#[derive(Debug)]
enum RangeCompare {
    Contains,
    Intersects,
    NoIntersection,
}
impl Range {
    fn compare(&self, other: &Range) -> RangeCompare {
        if (self.start >= other.start && self.end <= other.end)
            || (self.start <= other.start && self.end >= other.end)
        {
            RangeCompare::Contains
        } else if (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
        {
            RangeCompare::Intersects
        } else {
            RangeCompare::NoIntersection
        }
    }

    fn expand(&mut self, other: &Range) {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        self.start = start;
        self.end = end;
    }
}

fn main() {
    // let file_path = "./input/example";
    let file_path = "./input/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.trim();

    let mut ranges: Vec<Range> = Vec::new();
    // part 1
    // let mut ids: Vec<u64> = Vec::new();
    // let mut reading_ranges = true;
    for line in contents.lines() {
        // if reading_ranges {
        if line.is_empty() {
            // reading_ranges = false;
            // part 1
            // continue;
            // part 2
            break;
        }
        let range: Vec<&str> = line.split('-').collect();
        let mut iter = range.iter();
        let start: u64 = iter.next().unwrap().parse::<u64>().unwrap();
        let end: u64 = iter.next().unwrap().parse::<u64>().unwrap();

        ranges.push(Range { start, end });
        // }
        // part 1
        // else {
        //     ids.push(line.parse::<u64>().unwrap());
        // }
    }

    // part 1
    // let mut fresh = 0u32;
    // for id in ids {
    //     for range in &ranges {
    //         if range.contains(&id) {
    //             fresh += 1;
    //             break;
    //         }
    //     }
    // }

    // part 2
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut fresh_ranges_combined: Vec<Range> = Vec::new();
    let first_range = ranges.first().unwrap();
    fresh_ranges_combined.push(Range {
        start: first_range.start,
        end: first_range.end,
    });
    for range in ranges {
        let mut range_found = false;
        for fresh_range in fresh_ranges_combined.iter_mut() {
            match fresh_range.compare(&range) {
                RangeCompare::Contains | RangeCompare::Intersects => {
                    fresh_range.expand(&range);
                    range_found = true;
                    break;
                }
                _ => (),
            }
        }
        if !range_found {
            fresh_ranges_combined.push(Range {
                start: range.start,
                end: range.end,
            });
        }
    }

    let sum: usize = fresh_ranges_combined
        .iter()
        .map(|e| (e.start..=e.end).count())
        .sum();
    println!("{sum}");
}
