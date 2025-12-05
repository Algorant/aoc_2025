use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_05/input.txt"))
        .expect("Could not find input.txt")
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn parse(s: &str) -> Range {
        let (start, end) = s.split_once('-').unwrap();
        Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut parts = input.split("\n\n");

    let ranges: Vec<Range> = parts.next().unwrap().lines().map(Range::parse).collect();

    let values: Vec<u64> = parts
        .next()
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    (ranges, values)
}

fn is_valid(value: u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|r| r.contains(value))
}

fn main() {
    let input = read_input();
    let (ranges, values) = parse_input(&input);

    println!("Loaded {} ranges and {} values", ranges.len(), values.len());

    let valid_counts = values.iter().filter(|&&v| is_valid(v, &ranges)).count();

    println!("Part 1 \n");
    println!("Valid values: {}", valid_counts);
}
