use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_07/input.txt"))
        .expect("Could not find input.txt")
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> Option<usize> {
    grid.first()?.iter().position(|&c| c == 'S')
}

fn simulate(grid: &mut Vec<Vec<char>>) -> usize {
    let mut split_count = 0;

    let start_col = find_start(grid).expect("No start position found");

    // Use Hashet to handle duplicates
    let mut active_beams: HashSet<usize> = HashSet::new();
    active_beams.insert(start_col);

    let rows = grid.len();
    let cols = grid[0].len();

    // Process row by row
    for row in 1..rows {
        let mut new_beams: HashSet<usize> = HashSet::new();

        for &col in &active_beams {
            match grid[row][col] {
                '.' => {
                    // Beam continues down
                    grid[row][col] = '|';
                    new_beams.insert(col);
                }
                '^' => {
                    // Beam splits left and right (same row)
                    split_count += 1;

                    // Left beam
                    if col > 0 {
                        if grid[row][col - 1] == '.' {
                            grid[row][col - 1] = '|';
                        }
                        new_beams.insert(col - 1);
                    }
                    // Right beam
                    if col + 1 < cols {
                        if grid[row][col + 1] == '.' {
                            grid[row][col + 1] = '|';
                        }
                        new_beams.insert(col + 1);
                    }
                }
                '|' => {
                    // Already a beam here, continue
                    new_beams.insert(col);
                }
                _ => {}
            }
        }
        active_beams = new_beams;

        if active_beams.is_empty() {
            break;
        }
    }
    split_count
}

fn count_paths(grid: &[Vec<char>]) -> usize {
    let start_col = find_start(grid).expect("No start position found");

    let rows = grid.len();
    let cols = grid[0].len();

    // Each path is represented by its current column position
    // We track how many paths are at each column
    // Using a Vec of (column, path_count) or just count paths at each position

    let mut path_counts: HashMap<usize, usize> = HashMap::new();
    path_counts.insert(start_col, 1);

    for row in 1..rows {
        let mut new_counts: HashMap<usize, usize> = HashMap::new();

        for (&col, &count) in &path_counts {
            match grid[row][col] {
                '.' | '|' => {
                    // Path continues straigth down
                    *new_counts.entry(col).or_insert(0) += count;
                }
                '^' => {
                    // Each path splits in two
                    if col > 0 {
                        *new_counts.entry(col - 1).or_insert(0) += count;
                    }
                    if col + 1 < cols {
                        *new_counts.entry(col + 1).or_insert(0) += count;
                    }
                }
                _ => {}
            }
        }

        path_counts = new_counts;

        if path_counts.is_empty() {
            break;
        }
    }

    // Total paths = sum of all path counts at the bottom
    path_counts.values().sum()
}

fn main() {
    let input = read_input();
    let mut grid = parse_grid(&input);

    let splits = simulate(&mut grid);

    // Part 2
    let paths = count_paths(&grid);

    // Debug print final grid
    println!("=== Final Grid ===");
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }

    println!("Part 1 Total: {}\n", splits);
    println!("Part 2: {} unique paths", paths);
}
