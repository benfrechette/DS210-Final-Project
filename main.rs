mod calcs;
mod graphs;
mod test;
use graphs::Graph;
use std::{env, fs::File, io::prelude::*, vec};
fn main() {
    let args: Vec<String> = env::args().collect(); // Reads file path through standard input and creates an undirected graph
    let file_path = &args[1];
    let (n, edges) = read_file(file_path);
    let graph = Graph::create_undirected(n, &edges);

    let mut all_distances: Vec<u32> = vec![]; // Storage for all measured distances in a graph
    for i in 0..graph.n { // Combines each vector of distances into a single vector.
        let distances = calcs::compute_bfs(i, &graph);
        all_distances.extend(distances)
    }

    let total_avg_distance: u32 = calcs::average(all_distances.clone()); // Calculates the average integer for every measured distance between every vertex
    let median_distance: u32 = calcs::median(all_distances.clone()); // Calculates the median integer for every measured distance between every vertex

    println!( // Prints average distance across all vertices in the graph
        "The average distance between each node is: {}",
        total_avg_distance
    );
    println!( // Prints median distance across all vertices in the graph
        "The median distance between each node is: {}",
        median_distance
    )
}
fn read_file(path: &str) -> (usize, Vec<(usize, usize)>) { // Read file function taken from lecture notes. Slightly altered.
    let mut first_line = true;
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut num_edges = 0usize;
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        if first_line {
            num_edges = line_str.trim().parse::<usize>().unwrap();
            first_line = false;
        } else {
            let v: Vec<&str> = line_str.trim().split(' ').collect();
            let x: usize = v[0].parse::<usize>().unwrap();
            let y: usize = v[1].parse::<usize>().unwrap();
            result.push((x, y));
        }
    }
    return (num_edges, result);
}