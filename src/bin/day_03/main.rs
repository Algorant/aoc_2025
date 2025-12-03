use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_03/input.txt"))
        .expect("Could not find input.txt")
}

// Main logic, find largest two digit number that can be made from largest two single integers in string
fn largest_two_digit(line: &str) -> (u32, u32, u32) {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    // Find first occurence of largest digit (cannot be last position)
    let mut first_digit = 0;
    let mut first_pos = 0;

    for (i, &d) in digits.iter().enumerate() {
        if i >= digits.len() - 1 {
            break; // Can't be the last digit
        }
        if d > first_digit {
            first_digit = d;
            first_pos = i;
        }
    }

    // Find largest digit after first_pos
    let second_digit = digits[first_pos + 1..].iter().copied().max().unwrap_or(0);

    (first_digit, second_digit, first_digit * 10 + second_digit)
}

// Part 2, find 12 digit number
fn largest_n_digit(line: &str, n: usize) -> (Vec<u32>, u64) {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    let len = digits.len();
    let mut result_digits: Vec<u32> = Vec::with_capacity(n);
    let mut start_pos = 0;

    for i in 0..n {
        let digits_still_needed = n - i - 1; // How many more after this one
        let end_pos = len - digits_still_needed; // Can't go past this

        // Find largest digit in range (start_pos, end_pos)
        let mut best_digit = 0;
        let mut best_pos = start_pos;

        for pos in start_pos..end_pos {
            if digits[pos] > best_digit {
                best_digit = digits[pos];
                best_pos = pos;
            }
        }

        result_digits.push(best_digit);
        start_pos = best_pos + 1; // Do next search
    }

    // Convert to number
    let result: u64 = result_digits
        .iter()
        .fold(0u64, |acc, &d| acc * 10 + d as u64);

    (result_digits, result)
}

fn main() {
    let input = read_input();
    let mut total_p1: u32 = 0;
    let mut total_p2: u64 = 0;

    for (i, line) in input.lines().enumerate() {
        // Part 1
        let (first, second, result1) = largest_two_digit(line);
        total_p1 += result1;

        // Part 2: 12 digits
        let (digits, result2) = largest_n_digit(line, 12);
        total_p2 += result2;

        // Show line number, first x chars, and running total
        let preview: String = line.chars().take(20).collect();
        println!(
            "Line {}: {}... -> P1: {}{} = {} | P2: {:?} = {}",
            i + 1,
            preview,
            first,
            second,
            result1,
            digits,
            result2
        );
    }
    println!("\nPart 1: {}", total_p1);
    println!("Part 2: {}", total_p2);
}
