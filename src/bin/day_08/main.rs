use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_08/input.txt"))
        .expect("Could not find input.txt")
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

// For storing a pair of points and their distance
struct Edge {
    i: usize,
    j: usize,
    dist_sq: i64,
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
                z: parts[2],
            }
        })
        .collect()
}

fn distance_squared(a: &Point, b: &Point) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

fn compute_all_edges(points: &[Point]) -> Vec<Edge> {
    let mut edges = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            edges.push(Edge {
                i,
                j,
                dist_sq: distance_squared(&points[i], &points[j]),
            });
        }
    }
    // sort by distance
    edges.sort_by_key(|e| e.dist_sq);
    edges
}

// Use Union-Find to find connected components
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    // Find the root of the circuit containing x (with path compression)
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    // Unite two circuits, returns true if separate
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // already in same circuit
        }

        // Union by size (attach smaller tree to larger)
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    // Get size of circuit containing x
    fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

// Main algorithm
fn solve(points: &[Point], max_connections: usize) -> Vec<usize> {
    let edges = compute_all_edges(points);
    let mut uf = UnionFind::new(points.len());

    let mut connections_made = 0; // Will stop at 1000

    for edge in edges.iter().take(max_connections) {
        uf.union(edge.i, edge.j);
    }
    // // Process edges in order of distance
    // for edge in edges {
    //     // Union return true if they were in different circuits
    //     if uf.union(edge.i, edge.j) {
    //         connections_made += 1;
    //         if connections_made >= max_connections {
    //             break;
    //         }
    //     }
    // }

    println!("Parents: {:?}", &uf.parent[..20.min(uf.parent.len())]);
    println!("Sizes: {:?}", &uf.size[..20.min(uf.size.len())]);

    // Collect circuit sizes (only count each root once)
    let mut sizes: Vec<usize> = (0..points.len())
        .filter_map(|i| {
            let root = uf.find(i);
            if root == i { Some(uf.size[i]) } else { None }
        })
        .collect();

    // Sort descending
    sizes.sort_by(|a, b| b.cmp(a));
    sizes
}

fn main() {
    let input = read_input();
    let points = parse_points(&input);

    println!("Parsed {} points", points.len());

    let circuit_sizes = solve(&points, 1000); // Stop at 1000

    // Show top circuits (for debuggging)
    println!(
        "Top circuit sizes: {:?}",
        &circuit_sizes[..circuit_sizes.len().min(10)]
    );

    // Part 1: product of largest three
    let part1: usize = circuit_sizes.iter().take(3).product();
    println!("Part 1: {}", part1);
}
