use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_04/input.txt"))
        .expect("Could not find input.txt")
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    // All 8 directions (like a numpad around center)
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1), // Top row
        (0, -1),
        (0, 1), // Middle row
        (1, -1),
        (1, 0),
        (1, 1), // Bottom row
    ];

    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        // Check bounds
        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

// Part 2, removing the found @s

fn remove_pass(grid: &mut Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    // First collect all positions to remove (don't modify while iterating)
    let mut to_remove: Vec<(usize, usize)> = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' {
                let neighbors = count_neighbors(&grid, row, col);
                if neighbors < 4 {
                    to_remove.push((row, col));
                }
            }
        }
    }

    // Now remove them
    for (row, col) in &to_remove {
        grid[*row][*col] = '.';
    }

    to_remove.len()
}

fn part_2(grid: &mut Vec<Vec<char>>) -> usize {
    let mut total_removed = 0;
    let mut pass = 1;

    loop {
        let removed = remove_pass(grid);
        if removed == 0 {
            break; // No more to remove
        }

        println!("Pass {}: removed {} @ symbols", pass, removed);
        total_removed += removed;
        pass += 1;
    }

    total_removed
}

fn main() {
    let input = read_input();

    // Part 1: use original grid
    let grid = parse_grid(&input);
    let mut p1_count = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '@' {
                let neighbors = count_neighbors(&grid, row, col);
                if neighbors < 4 {
                    p1_count += 1;
                    // Debug output
                    println!(
                        "@ at ({}, {}) has {} neighbors - COUNTED",
                        row, col, neighbors
                    );
                }
            }
        }
    }

    println!("\nPart 1: {}\n", p1_count);

    // Part 2: Remove all @s
    let mut grid2 = parse_grid(&input);
    let p2_count = part_2(&mut grid2);

    println!("\nPart 2: {}", p2_count);
}
