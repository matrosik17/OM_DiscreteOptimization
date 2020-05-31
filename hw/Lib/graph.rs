mod graph {
    use std::ops::Index;

    pub struct Graph<N, E>
    {
        nodes: Vec<N>,
        edges: Vec<Vec<(usize, E)>>,
    }

    impl<N, E> Graph<N, E>
    where E: Copy,
    {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
            }
        }

        pub fn from_nodes(nodes: Vec<N>) -> Self {
            let n_nodes = nodes.len();
            Self {
                nodes,
                edges: vec![Vec::new(); n_nodes],
            }
        }

        pub fn size(&self) -> (usize, usize) {
            let n_nodes = self.nodes.len();
            let n_edges = self.edges.iter().map(|neigh| neigh.len()).sum();
            (n_nodes, n_edges)
        }

        pub fn neigh(&self, node_idx: usize) -> &[(usize, E)] {
            &self.edges[node_idx]
        }

        pub fn neigh_mut(&mut self, node_idx: usize) -> &mut Vec<(usize, E)> {
            &mut self.edges[node_idx]
        }

        pub fn add_node(&mut self, node: N) -> usize {
            self.nodes.push(node);
            self.edges.push(Vec::new());
            self.nodes.len() - 1
        }

        pub fn add_edge(&mut self, node1: usize, node2: usize, weight: E) {
            self.edges[node1].push((node2, weight));
        }

        pub fn add_edge_undirected(&mut self, node1: usize, node2: usize, weight: E) {
            self.edges[node1].push((node2, weight));
            self.edges[node2].push((node1, weight));
        }
    }

    // graph[i] - значение в вершине
    impl<N, E> Index<usize> for Graph<N, E> {
        type Output = N;

        fn index(&self, node_idx: usize) -> &Self::Output {
            &self.nodes[node_idx]
        }
    }

    // graph[(i, j)] - значение ребра (i, j)
    impl<N, E> Index<(usize, usize)> for Graph<N, E> {
        type Output = E;

        fn index(&self, edge_idx: (usize, usize)) -> &Self::Output {
            let (node1, node2) = edge_idx;
            let edge = self.edges[node1].iter().find(|(idx, _)| *idx == node2);
            let (_, weight) = edge.expect("Edge doesn't exist");
            weight
        }
    }
}