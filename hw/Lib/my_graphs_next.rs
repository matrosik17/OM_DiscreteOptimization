mod graphs {
    use std::ops::{Index, IndexMut};

    pub trait Graph<N, E>:
        Index<(usize, usize), Output=E>
        + Index<usize, Output=N>
        + IndexMut<(usize, usize)>
        + IndexMut<usize>
    {
        fn shape(&self) -> (usize, usize);
    }
}

mod full_graph {
    use matrix::Matrix;
    use graphs::Graph;
    use std::ops::{Index, IndexMut};

    pub struct FullGraph<N, E>
    where
        N: Default + Clone,
        E: Default + Clone,
    {
        nodes: Vec<N>,
        edges: Matrix<E>,
    }

    impl<N, E> FullGraph<N, E>
    where
        N: Default + Clone,
        E: Default + Clone,
    {
        pub fn new(num_nodes: usize) -> Self {
            let nodes = vec![N::default(); num_nodes];
            let edges = Matrix::empty_squared(num_nodes);
            Self { nodes, edges }
        }

        pub fn from_nodes(nodes: Vec<N>) -> Self {
            let num_nodes = nodes.len();
            let edges = Matrix::empty_squared(num_nodes);
            Self { nodes, edges }
        }
    }

    impl<N, E> Graph<N, E> for FullGraph<N, E>
    where
        N: Default + Clone,
        E: Default + Clone,
    {
        fn shape(&self) -> (usize, usize) {
            let n_nodes = self.nodes.len();
            let n_edges = {
                let (rows, cols) = self.edges.shape();
                rows * cols
            };
            (n_nodes, n_edges)
        }
    }

    impl<N, E> Index<(usize, usize)> for FullGraph<N, E>
    where
        N: Default + Clone,
        E: Default + Clone,
    {
        type Output = E;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.edges[index]
        }
    }

    impl<N, E> IndexMut<(usize, usize)> for FullGraph<N, E>
    where
        N: Default + Clone,
        E: Default + Clone,
    {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut E {
            &mut self.edges[index]
        }
    }

    impl<N, E> Index<usize> for FullGraph<N, E>
    where
        N: Default + Clone,
        E: Default + Clone,
    {
        type Output = N;

        fn index(&self, node_idx: usize) -> &Self::Output {
            &self.nodes[node_idx]
        }
    }

    impl<N, E> IndexMut<usize> for FullGraph<N, E>
    where
        N: Default + Clone,
        E: Default + Clone,
    {
        fn index_mut(&mut self, node_idx: usize) -> &mut N {
            &mut self.nodes[node_idx]
        }
    }
}