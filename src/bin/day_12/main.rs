use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_12/input.txt"))
        .expect("Could not find input.txt")
}

type Shape = Vec<(i32, i32)>; // (row, col) coordinates of # cells

// Parse 3x3 gift shape into coordinate list
fn parse_gift(lines: &[&str]) -> Shape {
    let mut coords = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                coords.push((r as i32, c as i32));
            }
        }
    }
    coords
}

// Rotate shape 90 degrees clockwise
fn rotate_90(shape: &Shape) -> Shape {
    shape.iter().map(|&(r, c)| (c, r)).collect()
}

// Flip horizontally
fn flip_h(shape: &Shape) -> Shape {
    shape.iter().map(|&(r, c)| (r, -c)).collect()
}

// Normalize shape to have min coords at origin (0,0)
fn normalize(shape: &Shape) -> Shape {
    if shape.is_empty() {
        return vec![];
    }
    let min_r = shape.iter().map(|&(r, _)| r).min().unwrap();
    let min_c = shape.iter().map(|&(_, c)| c).min().unwrap();
    shape.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect()
}

// Generate all possible rotations and flips (8 possible total)
fn generate_transformations(shape: &Shape) -> Vec<Shape> {
    let mut transformations = Vec::new();

    // Original + 4 rotations
    let mut current = normalize(shape);
    for _ in 0..4 {
        transformations.push(current.clone());
        current = rotate_90(&current);
    }

    // Flipped + 4 rotations
    let mut current = normalize(&flip_h(shape));
    for _ in 0..4 {
        transformations.push(current.clone());
        current = rotate_90(&current);
    }

    // Remove duplicates from above
    transformations.sort();
    transformations.dedup();
    transformations
}

// Check if shape can be placed at (r,c) on grid
fn can_place(grid: &[Vec<bool>], shape: &Shape, r: i32, c: i32) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for &(dr, dc) in shape {
        let nr = r + dr;
        let nc = c + dc;

        if nr < 0 || nr >= height || nc < 0 || nc >= width {
            return false; // out of bounds
        }

        if grid[nr as usize][nc as usize] {
            return false; // Cell already occupied
        }
    }
    true
}

fn place(grid: &mut [Vec<bool>], shape: &Shape, r: i32, c: i32) {
    for &(dr, dc) in shape {
        grid[(r + dr) as usize][(c + dc) as usize] = true;
    }
}

// Remove shape from grid (for backtracking)
fn unplace(grid: &mut [Vec<bool>], shape: &Shape, r: i32, c: i32) {
    for &(dr, dc) in shape {
        grid[(r + dr) as usize][(c + dc) as usize] = false;
    }
}

// Backtracking solver
fn solve(
    grid: &mut [Vec<bool>],
    gifts: &[(usize, Shape)], // (gift_id, shape)
    idx: usize,
    all_transformations: &[Vec<Shape>],
) -> bool {
    if idx >= gifts.len() {
        return true;
    }

    let (gift_id, _) = gifts[idx];

    // Try all transformations
    for transformation in &all_transformations[gift_id] {
        // Try all positions
        for r in 0..grid.len() as i32 {
            for c in 0..grid[0].len() as i32 {
                if can_place(grid, transformation, r, c) {
                    place(grid, transformation, r, c);

                    if solve(grid, gifts, idx + 1, all_transformations) {
                        return true;
                    }

                    unplace(grid, transformation, r, c);
                }
            }
        }
    }

    false
}

fn print_grid(grid: &[Vec<Option<char>>]) {
    for row in grid {
        for cell in row {
            print!("{}", cell.unwrap_or(' '));
        }
        println!();
    }
}

fn main() {
    let input = read_input();
    let lines: Vec<&str> = input.lines().collect();

    // Parse gift shapes
    let mut base_gifts = Vec::new();
    for i in 0..6 {
        let start = i * 5 + 1;
        let gift_lines = &lines[start..start + 3];
        base_gifts.push(parse_gift(gift_lines));
    }

    // Generate all transformations
    let all_transformations: Vec<Vec<Shape>> = base_gifts
        .iter()
        .map(|g| generate_transformations(g))
        .collect();

    // Process all trees
    let mut valid_count = 0;

    for (tree_idx, line) in lines.iter().skip(30).enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        let dims: Vec<usize> = parts[0]
            .split('x')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let width = dims[0];
        let height = dims[1];

        let counts: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Build gift list
        let mut gifts = Vec::new();
        for (gift_id, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                gifts.push((gift_id, base_gifts[gift_id].clone()));
            }
        }

        // Quick area check
        let total_cells: usize = gifts.iter().map(|(_, g)| g.len()).sum();
        if total_cells > width * height {
            continue;
        }

        // Try to solve
        let mut grid = vec![vec![false; width]; height];
        if solve(&mut grid, &gifts, 0, &all_transformations) {
            valid_count += 1;
            println!("Tree {} valid", tree_idx + 1);
        }
    }

    println!("\nTotal valid trees: {}", valid_count);
}
