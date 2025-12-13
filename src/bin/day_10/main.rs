use std::collections::{HashMap, HashSet, VecDeque};
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
// Needed insight from this brilliant reddit post:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

// Helper function
fn find_parity_patterns(buttons: &[Vec<usize>], target_parity: &[bool]) -> Vec<u32> {
    let n_buttons = buttons.len();
    let n_counters = target_parity.len();
    let mut valid_patterns = Vec::new();

    // Try all 2^n button combinations
    for mask in 0..(1 << n_buttons) {
        let mut state = vec![false; n_counters];

        for (btn_idx, button) in buttons.iter().enumerate() {
            if (mask & (1 << btn_idx)) != 0 {
                // Press this button
                for &pos in button {
                    state[pos] = !state[pos];
                }
            }
        }

        if state == target_parity {
            valid_patterns.push(mask);
        }
    }
    valid_patterns
}

// Recursive approach
fn solve_p2_recurse(
    buttons: &[Vec<usize>],
    targets: &[usize],
    cache: &mut HashMap<Vec<usize>, usize>,
) -> usize {
    // Base Case: all zeros
    if targets.iter().all(|&t| t == 0) {
        return 0;
    }

    // Check cache
    if let Some(&result) = cache.get(&targets.to_vec()) {
        return result;
    }

    // Calc target parity (odd = true, even = false)
    let target_parity: Vec<bool> = targets.iter().map(|&t| t % 2 == 1).collect();

    // Find all button patterns that achieve this parity
    let patterns = find_parity_patterns(buttons, &target_parity);

    if patterns.is_empty() {
        // IMPOSSIBLE
        cache.insert(targets.to_vec(), usize::MAX);
        return usize::MAX;
    }

    let mut min_presses = usize::MAX;

    for mask in patterns {
        let mut remaining = targets.to_vec();
        let mut presses_used = 0;
        let mut valid = true;

        // Apply buttons indicated by mask (press once each)
        for (btn_idx, button) in buttons.iter().enumerate() {
            if (mask & (1 << btn_idx)) != 0 {
                presses_used += 1;
                for &pos in button {
                    if remaining[pos] == 0 {
                        valid = false;
                        break;
                    }
                    remaining[pos] -= 1;
                }
                if !valid {
                    break;
                }
            }
        }

        if !valid {
            continue;
        }

        // Check if all remaining values are even
        if !remaining.iter().all(|&v| v % 2 == 0) {
            continue;
        }

        // Divide by 2 and recurse
        let halved: Vec<usize> = remaining.iter().map(|&v| v / 2).collect();
        let sub_result = solve_p2_recurse(buttons, &halved, cache);

        if sub_result != usize::MAX {
            min_presses = min_presses.min(presses_used + 2 * sub_result);
        }
    }

    cache.insert(targets.to_vec(), min_presses);
    min_presses
}

fn solve_p2(device: &Device) -> usize {
    let targets = device.joltage_targets.as_ref().unwrap();
    let mut cache = HashMap::new();
    solve_p2_recurse(&device.buttons, &targets, &mut cache)
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
            let min_presses_p2 = solve_p2(&device);
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
