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

fn main() {
    let input = read_input();
    let mut total: u32 = 0;

    for (i, line) in input.lines().enumerate() {
        let (first, second, result) = largest_two_digit(line);
        total += result;

        // Show line number, first x chars, and running total
        let preview: String = line.chars().take(20).collect();
        println!(
            "Line {}: {}... -> {} then {} = {} (total: {})",
            i + 1,
            preview,
            first,
            second,
            result,
            total
        );
    }
    println!("\nPart 1: {}", total);
}
