use std::fs::read_to_string;

fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_02/input.txt"))
        .expect("Could not find input.txt")
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn parse(s: &str) -> Range {
        let parts: Vec<&str> = s.split('-').collect();
        Range {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }

    // Returns an iterator over all numbers in the range
    fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..=self.end
    }
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input.trim().split(',').map(Range::parse).collect()
}

fn main() {
    let ranges = parse_ranges(&read_input());
    // Print each range on a newline
    for range in ranges {
        println!("{:?}", range);
    }
}
