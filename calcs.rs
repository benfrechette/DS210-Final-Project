use crate::graphs::{Graph, Vertex};
use std::collections::VecDeque;

pub fn compute_bfs(start: Vertex, graph: &Graph) -> Vec<u32> { // Alteration to example code used in lecture.
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] {
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    let mut distances: Vec<u32> = vec![]; 
    for v in 0..graph.n { // Iterates through the entire graph
        distances.push(distance[v].unwrap()); // Stores every value into a single vector
    }
    return distances;
}
pub fn average(distances: Vec<u32>) -> u32 { // Returns the integer average of a single vector
    let mut sum: u32 = 0;
    let length: u32 = distances.len().try_into().unwrap();
    for i in &distances {
        sum = sum + i
    }
    return sum / length;
}
pub fn median(mut distances: Vec<u32>) -> u32 { // Returns the integer median of a single vector
    distances.sort();
    let midpoint = distances.len() / 2;
    return distances[midpoint];
}