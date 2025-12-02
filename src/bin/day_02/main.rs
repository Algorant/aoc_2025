use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_02/input.txt"))
        .expect("Could not find input.txt")
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input.trim().split(',').map(Range::parse).collect()
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

// Detection logic
// Function for pattern detection
// rough steps: convert number to string, try all possible pattern lengths from 1 to len/2, then check if repeating that pattern exists

// Attempt 2 at repeating pattern detection, only need to check for repeat twice
fn is_repeating_pattern(n: u64) -> bool {
    let s = n.to_string(); // Convert to string
    let len = s.len(); // Get length

    // Must have even length to split in half
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;

    // First half must equal second half
    s[..half] == s[half..]
}

fn is_repeating_multiple(n: u64) -> bool {
    let s = n.to_string(); // Convert to string
    let len = s.len(); // Get length

    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &s[..pattern_len];
        let repeat_count = len / pattern_len;

        if pattern.repeat(repeat_count) == s {
            return true;
        }
    }
    false
}

fn find_pattern_in_range(range: &Range) -> Vec<u64> {
    range.iter().filter(|&n| is_repeating_pattern(n)).collect()
}

fn find_multiple_in_range(range: &Range) -> Vec<u64> {
    range.iter().filter(|&n| is_repeating_multiple(n)).collect()
}

fn main() {
    let ranges = parse_ranges(&read_input());
    // Keep running total of repeated numbers
    let mut total_p1: u64 = 0;
    let mut total_p2: u64 = 0;

    for range in ranges {
        let twice = find_pattern_in_range(&range);
        let multiple = find_multiple_in_range(&range);
        // println!("{:?} -> {:?}", range, repeating);
        total_p1 += twice.iter().sum::<u64>();
        total_p2 += multiple.iter().sum::<u64>();
    }
    println!("Part 1: {}", total_p1);
    println!("Part 2: {}", total_p2);
}
