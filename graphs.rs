pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

impl Graph { // Graph implementation taken from lecture notes. Some alterations made.
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Self::create_directed(n, edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g
    }
}
pub fn reverse_edges(list: &ListOfEdges) -> ListOfEdges {
    let mut new_list = vec![];
    for (u, v) in list {
        new_list.push((*v, *u));
    }
    new_list
}