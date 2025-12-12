use std::collections::HashMap;
use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_09/input.txt"))
        .expect("Could not find input.txt")
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
            }
        })
        .collect()
}

// Part 1: find max rectangle
fn find_max_rectangle(points: &[Point]) -> (Point, Point, i64) {
    let mut max_area = 0;
    let mut best_pair = (points[0], points[0]);

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            // Must be diagonal (different x and different y)
            if p1.x != p2.x && p1.y != p2.y {
                let width = (p1.x - p2.x).abs() + 1;
                let height = (p1.y - p2.y).abs() + 1;
                let area = width * height;

                if area > max_area {
                    max_area = area;
                    best_pair = (*p1, *p2);
                }
            }
        }
    }

    (best_pair.0, best_pair.1, max_area)
}

// Part 2: Needed help from reddit
// Rasterization with flood fill instead of ray casting

fn solve_p2(points: &[Point]) -> i64 {
    // Coordinate compression
    let mut uniq_x: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut uniq_y: Vec<i64> = points.iter().map(|p| p.y).collect();
    uniq_x.sort();
    uniq_x.dedup();
    uniq_y.sort();
    uniq_y.dedup();

    let x_map: HashMap<i64, usize> = uniq_x.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_map: HashMap<i64, usize> = uniq_y.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    // Build grid of points
    let height = uniq_y.len();
    let width = uniq_x.len();
    let mut grid = vec![vec!['.'; width]; height];

    // Rasterize polygon edges
    for i in 0..points.len() {
        let a = &points[i];
        let b = &points[(i + 1) % points.len()];

        let ax = x_map[&a.x];
        let ay = y_map[&a.y];
        let bx = x_map[&b.x];
        let by = y_map[&b.y];

        if ax == bx {
            // Vertical edge
            let (y1, y2) = if ay < by { (ay, by) } else { (by, ay) };
            for y in y1..=y2 {
                grid[y][ax] = '#';
            }
        } else if ay == by {
            // Horizontal edge
            let (x1, x2) = if ax < bx { (ax, bx) } else { (bx, ax) };
            for x in x1..=x2 {
                grid[ay][x] = '#';
            }
        }
    }

    println!("After rasterization:");
    print_grid(&grid, 20);

    // Flood fill
    let inside_pt = get_inside_point(&grid);
    flood_fill(&mut grid, inside_pt);

    println!("After flood fill:");
    print_grid(&grid, 20);

    // Check all rectangle pairs
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            if p1.x != p2.x && p1.y != p2.y {
                if is_enclosed(p1, p2, &grid, &x_map, &y_map) {
                    let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
                    max_area = max_area.max(area);
                }
            }
        }
    }
    max_area
}

fn flood_fill(grid: &mut Vec<Vec<char>>, start: Point) {
    let mut stack = vec![start];
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(p) = stack.pop() {
        if grid[p.y as usize][p.x as usize] != '.' {
            continue;
        }
        grid[p.y as usize][p.x as usize] = 'X';

        for (dx, dy) in &dirs {
            let nx = p.x + dx;
            let ny = p.y + dy;
            if ny >= 0
                && (ny as usize) < grid.len()
                && nx >= 0
                && (nx as usize) < grid[0].len()
                && grid[ny as usize][nx as usize] == '.'
            {
                stack.push(Point { x: nx, y: ny });
            }
        }
    }
}

fn get_inside_point(grid: &Vec<Vec<char>>) -> Point {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '.' {
                continue;
            }

            let mut hits = 0;
            let mut prev = '.';

            for i in (0..=x).rev() {
                let cur = grid[y][i];
                if cur != prev {
                    hits += 1;
                }
                prev = cur;
            }

            if hits % 2 == 1 {
                return Point {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
    }
    panic!("Could not find inside point");
}

fn is_enclosed(
    a: &Point,
    b: &Point,
    grid: &Vec<Vec<char>>,
    x_map: &HashMap<i64, usize>,
    y_map: &HashMap<i64, usize>,
) -> bool {
    let x1 = x_map[&a.x.min(b.x)];
    let x2 = x_map[&a.x.max(b.x)];
    let y1 = y_map[&a.y.min(b.y)];
    let y2 = y_map[&a.y.max(b.y)];

    // Check top and bottom edges
    for x in x1..=x2 {
        if grid[y1][x] == '.' || grid[y2][x] == '.' {
            return false;
        }
    }

    // Check left and right edges
    for y in y1..=y2 {
        if grid[y][x1] == '.' || grid[y][x2] == '.' {
            return false;
        }
    }
    true
}

fn print_grid(grid: &Vec<Vec<char>>, max_rows: usize) {
    let rows = grid.len().min(max_rows);
    for i in 0..rows {
        println!("{}", grid[i].iter().collect::<String>());
    }
    if grid.len() > max_rows {
        println!("... ({} more rows)", grid.len() - max_rows);
    }
}

fn main() {
    let input = read_input();
    let points = parse_points(&input);

    // Part 1
    println!("=== Part 1 ===");
    let (p1, p2, area) = find_max_rectangle(&points);
    println!("Point 1: ({}, {})", p1.x, p1.y);
    println!("Point 2: ({}, {})", p2.x, p2.y);
    println!("Area: {}", area);

    // Part 2
    println!("=== Part 2 ===");
    let area_p2 = solve_p2(&points);
    println!("Area: {}", area_p2);
}
