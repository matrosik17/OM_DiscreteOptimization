use io::Scanner;
use full_graph::FullGraph;
use std::cmp::Ordering;

type Point = (i64, f64, f64);
type Node = i64;
type Edge = f64;
type FGraph = FullGraph<Node, Edge>;

fn distance(p1: &Point, p2: &Point) -> Edge {
    let (_, x1, y1) = p1;
    let (_, x2, y2) = p2;
    ((x2 - x1).powf(2.) + (y2 - y1).powf(2.)).sqrt()
}

fn build_full_graph(points: &[Point]) -> FGraph {
    let nodes: Vec<Node> = points.iter()
        .map(|(p, _, _)| *p)
        .collect();

    let mut graph = FGraph::from_nodes(nodes);
    for (i, pi) in points.iter().enumerate() {
        for (j, pj) in points.iter().enumerate() {
            graph[(i, j)] = distance(pi, pj);
        }
    }
    graph
}

fn build_mst(graph: &FGraph) -> Vec<(usize, usize)> {
    let (n_nodes, _) = graph.shape();

    let mut in_mst_nodes = vec![false; n_nodes];

    in_mst_nodes[0] = true;
    let mut edge_ends = vec![0_usize; n_nodes];
    let mut edge_weights: Vec<Edge> = (0..n_nodes)
        .map(|node_idx| graph[(0, node_idx)])
        .collect();

    for _ in 1..n_nodes {
        let (new_mst_node_idx, _) = edge_weights.iter()
            .enumerate()
            .filter(|(node_idx, _)| !in_mst_nodes[*node_idx])
            .min_by(|(_, w1), (_, w2)| match w1.partial_cmp(w2) {
                Some(ord) => ord,
                None => Ordering::Less,
            })
            .unwrap();

        in_mst_nodes[new_mst_node_idx] = true;
        for j in 0..n_nodes {
            if in_mst_nodes[j] { continue; }
            else {
                if graph[(j, new_mst_node_idx)] < edge_weights[j] {
                    edge_weights[j] = graph[(j, new_mst_node_idx)];
                    edge_ends[j] = new_mst_node_idx;
                }
            }
        }
    }

    let edges = edge_ends.into_iter()
        .enumerate()
        .filter_map(|edge| match edge {
            (i, j) if i == j => None,
            (i, j) => Some((i, j)),
        })
        .collect();
    edges
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let points: Vec<Point> = (0..n)
        .map(|_| (scan.token(), scan.token(), scan.token()))
        .collect();

    let graph = build_full_graph(&points);
    println!("{:?}", graph.shape());

    let mst_edges = build_mst(&graph);
    println!("{:?}", mst_edges);
}

mod io {
    use std::io::BufRead;

    pub struct Scanner<B> {
        reader: B,
        buf_str: String,
        buf_iter: std::str::SplitWhitespace<'static>,
    }

    impl<B: BufRead> Scanner<B> {
        pub fn new(reader: B) -> Self {
            Self {
                reader,
                buf_str: String::new(),
                buf_iter: "".split_whitespace(),
            }
        }
        pub fn token<T: std::str::FromStr>(&mut self) -> T {
            loop {
                if let Some(token) = self.buf_iter.next() {
                    return token.parse().ok().expect("Failed parse");
                }
                self.buf_str.clear();
                self.reader
                    .read_line(&mut self.buf_str)
                    .expect("Failed read");
                self.buf_iter = unsafe { std::mem::transmute(self.buf_str.split_whitespace()) };
            }
        }
    }

}

mod matrix {
    use std::ops::{Index, IndexMut};

    pub struct Matrix<T: Default + Clone> {
        rows: usize,
        columns: usize,
        elements: Vec<T>,
    }

    impl<T: Default + Clone> Matrix<T> {
        pub fn new(rows: usize, columns: usize, elements: Vec<T>) -> Self {
            Self { rows, columns, elements }
        }

        pub fn empty(rows: usize, columns: usize) -> Self {
            let elements = vec![T::default(); rows * columns];
            Self::new(rows, columns, elements)
        }

        pub fn empty_squared(size: usize) -> Self {
            Self::empty(size, size)
        }

        pub fn shape(&self) -> (usize, usize) {
            (self.rows, self.columns)
        }
    }

    impl<T: Default + Clone> Index<(usize, usize)> for Matrix<T> {
        type Output = T;

        fn index(&self, index2d: (usize, usize)) -> &Self::Output {
            let (row_idx, col_idx) = index2d;
            let element_idx = row_idx * self.columns + col_idx;
            &self.elements[element_idx]
        }
    }

    impl<T: Default + Clone> IndexMut<(usize, usize)> for Matrix<T> {
        fn index_mut(&mut self, index2d: (usize, usize)) -> &mut T {
            let (row_idx, col_idx) = index2d;
            let element_idx = row_idx * self.columns + col_idx;
            &mut self.elements[element_idx]
        }
    }
}

mod full_graph {
    use matrix::Matrix;
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

        pub fn shape(&self) -> (usize, usize) {
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