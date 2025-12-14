use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

// Helper functions
fn read_input() -> String {
    // Try relative path first (for running from bin directory)
    // Then try from project root
    read_to_string("input.txt")
        .or_else(|_| read_to_string("src/bin/day_11/input.txt"))
        .expect("Could not find input.txt")
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        let source = parts[0].trim().to_string();

        // Separated by whitespace, not comma
        let destinations: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect();

        graph.insert(source, destinations);
    }
    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashSet<String>,
) -> usize {
    // Base case: reached target
    if current == target {
        return 1;
    }

    // Check if already visited before marking
    if visited.contains(current) {
        return 0;
    }

    // Mark current node as visited
    visited.insert(current.to_string());

    let mut total_paths = 0;

    // Explore all neighbors
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                total_paths += count_paths(graph, neighbor, target, visited);
            }
        }
    }

    // Backtrack: unmark current node
    visited.remove(current);

    total_paths
}

// Part 2 DFS with Memoization
fn dfs_memo(
    device: &str,
    visited_fft: bool,
    visited_dac: bool,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    // Base case: reached target
    if device == "out" {
        return if visited_fft && visited_dac { 1 } else { 0 };
    }

    // Check memo
    let key = (device.to_string(), visited_fft, visited_dac);

    if let Some(&result) = memo.get(&key) {
        return result;
    }

    // Update waypoint flags for next iteration
    let next_fft = visited_fft || device == "fft";
    let next_dac = visited_dac || device == "dac";

    // Recurse on  all neighbors
    let mut paths_count = 0;
    if let Some(neighbors) = graph.get(device) {
        for next_device in neighbors {
            paths_count += dfs_memo(next_device, next_fft, next_dac, graph, memo);
        }
    }

    // Cache result
    memo.insert(key, paths_count);
    paths_count
}

fn main() {
    let input = read_input();
    let graph = parse_graph(&input);

    // Part 1
    // Print graph for debug
    println!("Graph:");
    let mut visited = HashSet::new();
    let p1 = count_paths(&graph, "you", "out", &mut visited);
    println!("Part 1: {}", p1);

    // Debug p2 memoization using p1
    // let mut visited_test = HashSet::new();
    // let svr_to_out = count_paths(&graph, "svr", "out", &mut visited_test);
    // println!("Total paths svr to out (no waypoints): {}", svr_to_out);

    // Part 2
    let mut memo: HashMap<(String, bool, bool), usize> = HashMap::new();
    let p2 = dfs_memo("svr", false, false, &graph, &mut memo);
    println!("Part 2: {}", p2);
}
