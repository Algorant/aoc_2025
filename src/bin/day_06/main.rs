use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_06/input.txt"))
        .expect("Could not find input.txt")
}

fn parse_input(input: &str) -> (Vec<&str>, &str) {
    let lines: Vec<&str> = input.lines().collect();
    // First 4 lines are numbers, 5th line is operators
    let number_lines = lines[0..4].to_vec();
    let operator_line = lines[4];
    (number_lines, operator_line)
}

fn find_column_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    // Find max line length
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // For each position, check if all lines have a space there
    let is_separator: Vec<bool> = (0..max_len)
        .map(|i| {
            lines
                .iter()
                .all(|line| line.as_bytes().get(i).map(|&b| b == b' ').unwrap_or(true))
        })
        .collect();

    // Now find contiguous runs of 'false' (non-separator = column data)
    let mut ranges = Vec::new();
    let mut start: Option<usize> = None;

    for (i, &is_sep) in is_separator.iter().enumerate() {
        match (is_sep, start) {
            (false, None) => start = Some(i), // column start
            (true, Some(s)) => {
                // column end
                ranges.push((s, i));
                start = None;
            }
            _ => {}
        }
    }
    // Last column extends to the end
    if let Some(s) = start {
        ranges.push((s, max_len))
    }
    ranges
}

fn extract_column_values(lines: &[&str], start: usize, end: usize) -> Vec<i64> {
    lines
        .iter()
        .filter_map(|line| {
            let slice = line.get(start..end).unwrap_or("");
            let trimmed = slice.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<i64>().ok()
            }
        })
        .collect()
}

fn extract_operator(line: &str, start: usize, end: usize) -> char {
    let slice = line.get(start..end).unwrap_or(" ");
    slice.chars().find(|&c| c != ' ').unwrap_or('+')
}

fn apply_operation(values: &[i64], op: char) -> i64 {
    if values.is_empty() {
        return 0;
    }
    values.iter().skip(1).fold(values[0], |acc, &val| match op {
        '+' => acc + val,
        '*' => acc * val,
        _ => panic!("Unknown operator: {}", op),
    })
}

// Part 2, extracting numbers right to left, going down each column
fn extract_column_values_rtl(lines: &[&str], start: usize, end: usize) -> Vec<i64> {
    let slices: Vec<&str> = lines
        .iter()
        .filter_map(|line| {
            let slice = line.get(start..end).unwrap_or("");
            if slice.trim().is_empty() {
                None
            } else {
                Some(slice)
            }
        })
        .collect();

    let col_width = end - start;
    let mut values = Vec::new();

    // Iterate positions from right to left within the columns
    for pos in (0..col_width).rev() {
        let mut digits = String::new();
        for slice in &slices {
            if let Some(c) = slice.chars().nth(pos) {
                if c.is_ascii_digit() {
                    digits.push(c);
                }
            }
        }
        if !digits.is_empty() {
            values.push(digits.parse::<i64>().unwrap());
        }
    }
    values
}

fn main() {
    let input = read_input();
    let (number_lines, operator_line) = parse_input(&input);
    let column_ranges = find_column_ranges(&number_lines);

    // Part 1
    let mut total_p1: i64 = 0;
    println!("=== Part 1 ===");
    for (i, &(start, end)) in column_ranges.iter().enumerate() {
        let values = extract_column_values(&number_lines, start, end);
        let op = extract_operator(operator_line, start, end);
        let result = apply_operation(&values, op);
        // Debug output
        println!("Column {}: {:?} {} = {}", i, values, op, result);
        total_p1 += result;
    }
    println!("Part 1 Total: {}\n", total_p1);

    // Part 2
    let mut total_p2: i64 = 0;
    println!("=== Part 2 ===");
    for (i, &(start, end)) in column_ranges.iter().enumerate() {
        let values = extract_column_values_rtl(&number_lines, start, end);
        let op = extract_operator(operator_line, start, end);
        let result = apply_operation(&values, op);
        println!("Column {}: {:?} {} = {}", i, values, op, result);
        total_p2 += result;
    }
    println!("Part 2 Total: {}\n", total_p2);
}
