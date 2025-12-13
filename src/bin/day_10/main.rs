use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_10/input.txt"))
        .expect("Could not find input.txt")
}

#[derive(Debug)]
struct Device {
    target: Vec<bool>,                   // Target state, true = #, false = .
    buttons: Vec<Vec<usize>>,            // Each button = list of bit positions to flip
    joltage_targets: Option<Vec<usize>>, // Part 2 = list of joltage targets
}

fn parse_line(line: &str) -> Device {
    // Extract target state from [....]
    let start_bracket = line.find('[').unwrap();
    let end_bracket = line.find(']').unwrap();
    let target_str = &line[start_bracket + 1..end_bracket];
    let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

    // Extract buttons from (...) (...)
    let buttons_section = &line[end_bracket + 1..];
    let mut buttons = Vec::new();

    for part in buttons_section.split(')') {
        if let Some(paren_start) = part.find('(') {
            let nums_str = &part[paren_start + 1..];
            let positions: Vec<usize> = nums_str
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect();
            if !positions.is_empty() {
                buttons.push(positions);
            }
        }
    }

    // Extract joltage targets from {....} for part 2
    let joltage_targets = if let Some(curly_start) = line.find('{') {
        let curly_end = line.find('}').unwrap();
        let nums_str = &line[curly_start + 1..curly_end];
        Some(
            nums_str
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect(),
        )
    } else {
        None
    };

    Device {
        target,
        buttons,
        joltage_targets,
    }
}

// Helper functions
fn apply_button(state: &[bool], button: &[usize]) -> Vec<bool> {
    let mut new_state = state.to_vec();
    for &pos in button {
        new_state[pos] = !new_state[pos] // Flip the bit
    }
    new_state
}

fn state_to_key(state: &[bool]) -> u64 {
    state.iter().enumerate().fold(
        0u64,
        |acc, (i, &bit)| {
            if bit { acc | (1 << i) } else { acc }
        },
    )
}

// BFS solver
fn solve_device(device: &Device) -> usize {
    let n = device.target.len();
    let start_state = vec![false; n]; // All off

    // Quick check: if already at target
    if start_state == device.target {
        return 0;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start_state.clone(), 0)); // state, num_presses
    visited.insert(state_to_key(&start_state));

    while let Some((state, presses)) = queue.pop_front() {
        // Try pressing each button
        for button in &device.buttons {
            let next_state = apply_button(&state, button);

            // Check if reached target
            if next_state == device.target {
                return presses + 1;
            }

            let key = state_to_key(&next_state);
            if !visited.contains(&key) {
                visited.insert(key);
                queue.push_back((next_state, presses + 1));
            }
        }
    }

    usize::MAX // No solution found
}

// Part 2, joltage counters
fn solve_part2(device: &Device) -> usize {
    let targets = device.joltage_targets.as_ref().unwrap();

    // If all targets are 0, no presses needed
    if targets.iter().all(|&t| t == 0) {
        return 0;
    }

    // Use iterative deepening: try limits 10..200
    for limit in (10..=200).step_by(10) {
        let result = solve_greedy(device, targets, &vec![0; targets.len()], 0, limit);
        if result != usize::MAX {
            return result;
        }
    }
    usize::MAX
}

// Greedy algo
fn solve_greedy(
    device: &Device,
    targets: &[usize],
    current: &[usize],
    presses: usize,
    max_presses: usize,
) -> usize {
    // Check if reached target
    if current == targets {
        return presses;
    }

    // Check if exceeded limit
    if presses > max_presses {
        return usize::MAX;
    }

    // Find best button to presses
    // Heuristc: choose button that makes most progress without exceeding
    let mut best_result = usize::MAX;

    for button in &device.buttons {
        let mut next = current.to_vec();
        let mut valid = true;
        let mut makes_progress = false;

        for &pos in button {
            next[pos] += 1;
            if next[pos] > targets[pos] {
                valid = false;
                break;
            }
            if current[pos] < targets[pos] {
                makes_progress = true;
            }
        }
        if valid && makes_progress {
            let result = solve_greedy(device, targets, &next, presses + 1, max_presses);
            best_result = best_result.min(result);

            // Early exit if found solution
            if best_result != usize::MAX {
                return best_result;
            }
        }
    }
    best_result
}

fn main() {
    let input = read_input();

    let mut total_p1 = 0;
    let mut total_p2 = 0;

    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let device = parse_line(line);

        // Part 1: indicator lights
        let min_presses_p1 = solve_device(&device);

        if min_presses_p1 == usize::MAX {
            println!("Device {}: Not solvable", i + 1);
        } else {
            println!("Device {}: Minimum presses = {}", i + 1, min_presses_p1);
            total_p1 += min_presses_p1;
        }

        // Part 2: Joltage counters
        if device.joltage_targets.is_some() {
            let min_presses_p2 = solve_part2(&device);
            if min_presses_p2 != usize::MAX {
                println!("Device {} (Part 2): {} presses", i + 1, min_presses_p2);
                total_p2 += min_presses_p2;
            } else {
                println!("Device {} (Part 2): IMPOSSIBLE", i + 1);
            }
        }
    }
    // Sum all the min presses
    println!("\n===Part 1 ===");
    println!("\nTotal Presses: {}", total_p1);
    println!("\n===Part 2 ===");
    println!("\nTotal Presses: {}", total_p2);
}
