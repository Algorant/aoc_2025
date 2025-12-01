use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_01/input.txt"))
        .expect("Could not find input.txt")
}

fn wrap_position(position: i32) -> i32 {
    ((position % 100) + 100) % 100
}

fn parse_line(line: &str) -> (char, i32) {
    let line = line.trim(); // Parse line with trimming
    let direction = line.chars().next().unwrap();
    let distance = line[1..].parse::<i32>().unwrap();
    (direction, distance)
}

// Dial stuff
struct Dial {
    current: i32,
    zero_count: i32,
    zero_passes: i32,
}

impl Dial {
    // Initial conditions
    fn new(starting_position: i32) -> Dial {
        Dial {
            current: starting_position,
            zero_count: 0,
            zero_passes: 0,
        }
    }

    fn update(&mut self, direction: char, distance: i32) {
        let old_position = self.current;

        // Part 1: Complete cycles (crosses 0 once)
        let complete_cycles = distance / 100;
        let remainder = distance % 100;

        // Part 2: Calculate new position
        let new_position = match direction {
            'L' => wrap_position(old_position - distance),
            'R' => wrap_position(old_position + distance),
            _ => old_position,
        };

        self.current = new_position;

        // Part 3: Count zero crossings
        let mut zero_crossings = complete_cycles;

        // Check if remainder movement crosses zero
        let mut remainder_crossing = 0;
        match direction {
            'R' => {
                // Moving right: cross 0 if old_pos + remainder >= 100
                if old_position + remainder >= 100 {
                    remainder_crossing = 1;
                }
            }
            'L' => {
                // Moving left: cross 0 if old_pos - remainder < 0
                if old_position <= remainder {
                    remainder_crossing = 1;
                }
            }
            _ => {}
        }

        // Special case: if we START at 0, don't count that as passing through
        if old_position == 0 && remainder_crossing > 0 {
            remainder_crossing = 0;
        }

        // Add remainder crossings total
        zero_crossings += remainder_crossing;

        // Part 4: Separate landing vs passing
        if new_position == 0 {
            self.zero_count += 1;
            // One crossing was landing, not passing
            if zero_crossings > 0 {
                self.zero_passes += zero_crossings - 1;
            }
        } else {
            self.zero_passes += zero_crossings;
        }
    }

    // Result method for storing/debugging
    fn result(&self) -> (i32, i32, i32) {
        (self.current, self.zero_count, self.zero_passes)
    }
}

// Solution logic
fn main() {
    // Create dial and starting conditions of being at 50
    let mut dial = Dial::new(50);

    // Create a vector to store the results
    let mut results = Vec::new();

    // Iterate over the input
    for line in read_input().lines() {
        let (direction, distance) = parse_line(line); // Parse the line into a tuple

        // Store state before update
        let before_position = dial.current;
        let before_count = dial.zero_count;
        let before_passes = dial.zero_passes;

        dial.update(direction, distance); // Update the dial

        // Calculate what changed in this line
        let after_position = dial.current;
        let count_delta = dial.zero_count - before_count;
        let passes_delta = dial.zero_passes - before_passes;

        // Print debug info

        println!(
            "{}{}:  {} -> {} | landed = {}, passed = {} | total_landed: {}, total_passed: {}",
            direction,
            distance,
            before_position,
            after_position,
            count_delta,
            passes_delta,
            dial.zero_count,
            dial.zero_passes
        );

        results.push(dial.result());
    }

    // Print results
    println!("Part 1");
    println!("Final position: {}", dial.current);
    println!("The dial goes to 0: {} times", dial.zero_count);

    println!("\nPart 2");
    println!(
        "The dial passes through 0 (during movement): {} times",
        dial.zero_passes
    );
    println!(
        "Total (landing + passing): {}",
        dial.zero_count + dial.zero_passes
    );

    // // Use input function and return first 5 lines
    // let input = read_input();
    // let lines: Vec<&str> = input.lines().collect();
    // let output: Vec<&str> = lines[0..5].iter().map(|line| line.trim()).collect();
    //
    // // Print output
    // println!("The first 5 lines of the input are:\n");
    // println!("{:?}", output);
}

// Test logic
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample_input() {
        let test_input = "\
                          R50
                          L10
                          R60
                          L25
                          R75";

        // Create dial starting at 50
        let mut dial = Dial::new(50);

        // Process each line
        for line in test_input.lines() {
            let line = line.trim(); // Remove leading/trailing whitespace in test
            if line.is_empty() {
                continue;
            } // Skip empty lines
            println!("Line: '{}'", line);
            let (direction, distance) = parse_line(line);
            dial.update(direction, distance);
        }

        // Assert expected results
        assert_eq!(dial.current, 0);
        assert_eq!(dial.zero_count, 2);
    }

    #[test]
    fn test_wrap_position() {
        assert_eq!(wrap_position(0), 0);
        assert_eq!(wrap_position(99), 99);
        assert_eq!(wrap_position(100), 0);
        assert_eq!(wrap_position(-1), 99);
        assert_eq!(wrap_position(-5), 95);
        assert_eq!(wrap_position(150), 50);
    }
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("R50"), ('R', 50));
        assert_eq!(parse_line("L10"), ('L', 10));
    }

    #[test]
    fn test_part2_example() {
        let test_input = "\
                          L68\n\
                          L30\n\
                          R48\n\
                          L5\n\
                          R60\n\
                          L55\n\
                          L1\n\
                          L99\n\
                          R14\n\
                          L82";

        // Create dial starting at 50
        let mut dial = Dial::new(50);

        // Process each line
        for line in test_input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (direction, distance) = parse_line(line);
            dial.update(direction, distance);
        }

        // Expected: 3 times landing on 0, 3 times passing through 0
        // Total for Part 2 = 6
        assert_eq!(dial.zero_count, 3, "Should land on 0 exactly 3 times");
        assert_eq!(dial.zero_passes, 3, "Should pass through 0 exactly 3 times");
    }
}
