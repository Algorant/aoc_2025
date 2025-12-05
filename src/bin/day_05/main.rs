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

// Part 2 logic
fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort by start value
    let mut sorted: Vec<Range> = ranges
        .iter()
        .map(|r| Range {
            start: r.start,
            end: r.end,
        })
        .collect();
    sorted.sort_by_key(|r| r.start);

    let mut merged: Vec<Range> = vec![];
    let mut current = Range {
        start: sorted[0].start,
        end: sorted[0].end,
    };

    for range in sorted.iter().skip(1) {
        if range.start <= current.end {
            // Overlapping or adjacent - Extend
            current.end = current.end.max(range.end);
        } else {
            // GAp - push current and start new
            merged.push(current);
            current = Range {
                start: range.start,
                end: range.end,
            };
        }
    }

    merged.push(current);

    merged
}

fn count_integers(ranges: &[Range]) -> u64 {
    ranges.iter().map(|r| r.end - r.start + 1).sum()
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

    // Part 2
    let merged = merge_ranges(&ranges);
    let total_integers = count_integers(&merged);

    println!("\nPart 2\n");
    println!("merged into {} ranges", merged.len());
    println!("Total valid products: {}", total_integers);
}
