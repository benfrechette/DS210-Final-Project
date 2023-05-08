#[cfg(test)] 
mod test { // Contains tests for each new and altered function
    use crate::calcs::average;
    use crate::calcs::median;

    #[test]
    fn median_test() {
        let test_vector = vec![0, 1, 2, 3, 4, 5, 6];
        let result = median(test_vector);
        assert_eq!(result, 3)
    }
    #[test]
    fn average_test() {
        let test_vector = vec![0, 1, 2, 3, 4, 5, 6];
        let result = average(test_vector);
        assert_eq!(result, 3)
    }
    #[test]
    fn compute_bfs_avg_test() {
        let test_vector = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
        let test_graph = crate::graphs::Graph::create_undirected(10, &test_vector);
        let mut results = vec![];
        for i in 0..test_graph.n {
            let bfs = crate::calcs::compute_bfs(i, &test_graph);
            results.extend(bfs)
        }
        let result = average(results);
        assert_eq!(result, 2)
    }
    #[test]
    fn reverse_edges_test() {
        let test_vector = vec![(0,1),(2,4),(5,6),(4,6),(8,7)];
        let result = crate::graphs::reverse_edges(&test_vector);
        assert_eq!(result, [(1,0),(4,2),(6,5),(6,4),(7,8)])
    }
}