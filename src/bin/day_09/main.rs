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

// Part 2: Raycasting algorightm?? cool!
fn find_max_rectangle_in_polygon(points: &[Point]) -> (Point, Point, i64) {
    let mut max_area = 0;
    let mut best_pair = (points[0], points[0]);

    // Try all pairs of input points
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            // Must be valid rectangle (diagonal point corners)
            if p1.x != p2.x && p1.y != p2.y {
                let corners = [
                    Point { x: p1.x, y: p1.y },
                    Point { x: p1.x, y: p2.y },
                    Point { x: p2.x, y: p1.y },
                    Point { x: p2.x, y: p2.y },
                ];

                if corners.iter().all(|c| is_inside_polygon(c, points)) {
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
    }

    (best_pair.0, best_pair.1, max_area)
}

// Check if points in polygon
fn is_inside_polygon(point: &Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n; // Next vertex (wraps around)
        let pi = &polygon[i];
        let pj = &polygon[j];

        // Ray casting: count intersections with edges
        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
    }
    inside
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
    let (p1_p2, p2_p2, area_p2) = find_max_rectangle_in_polygon(&points);
    println!("Point 1: ({}, {})", p1_p2.x, p1_p2.y);
    println!("Point 2: ({}, {})", p2_p2.x, p2_p2.y);
    println!("Area: {}", area_p2);
}
