use io::Scanner;
use matrix::Matrix;
use graphs::Graph;
use full_graph::FullGraph;
use std::ops::Index;

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
    let num_nodes = points.len();
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

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let points: Vec<Point> = (0..n)
        .map(|_| (scan.token(), scan.token(), scan.token()))
        .collect();
    println!("{:?}", points);

    let mut matrix = Matrix::new(10, 10, vec![5; 100]);
    let vec: Vec<usize> = matrix.row_mut(1)
        .map(|x| *x)
        .collect();
    println!("{:?}", vec);

    // let graph = build_full_graph(&points);
    // println!("{:?}", graph.shape());
    // let graph = FullGraph::new(&points);
    // println!("{:?}", graph.edges.elements);

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
    use std::slice::{Iter, IterMut};
    use std::ops::{Index, IndexMut};

    pub struct Matrix<T: Default + Clone> {
        rows: usize,
        columns: usize,
        elements: Vec<T>,
    }

    pub struct RowIter<'a, T>(Iter<'a, T>) where T: Default + Clone;

    impl<'a, T> Iterator for RowIter<'a, T>
    where
    T: Default + Clone,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    pub struct RowIterMut<'a, T>(IterMut<'a, T>) where T: Default + Clone;

    impl<'a, T> Iterator for RowIterMut<'a, T>
    where
        T: Default + Clone,
    {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    pub struct ColumnIter<'a, T>
    where
        T: Default + Clone,
    {
        row_idx: usize,
        col_idx: usize,
        matrix: &'a Matrix<T>,
    }

    impl<'a, T> Iterator for ColumnIter<'a, T>
    where
        T: Default + Clone,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.row_idx < self.matrix.rows {
                let index = (self.row_idx, self.col_idx);
                self.row_idx += 1;
                Some(&self.matrix[index])
            } else {
                None
            }
        }
    }

    // pub struct ColumnIterMut<'a, T>
    // where
    //     T: Default + Clone,
    // {
    //     row_idx: usize,
    //     col_idx: usize,
    //     matrix: &'a mut Matrix<T>,
    // }

    // impl<'a, T> Iterator for ColumnIterMut<'a, T>
    // where
    //     T: Default + Clone,
    // {
    //     type Item = &'a mut T;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.row_idx < self.matrix.rows {
    //             let index = (self.row_idx, self.col_idx);
    //             self.row_idx += 1;
    //             Some(&mut self.matrix[index])
    //         } else {
    //             None
    //         }
    //     }
    // }

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

        pub fn row(&self, row_idx: usize) -> RowIter<T> {
            let start_idx = row_idx * self.columns;
            let stop_idx = start_idx + self.columns;
            RowIter(self.elements[start_idx..stop_idx].iter())
        }

        pub fn row_mut(&mut self, row_idx: usize) -> RowIterMut<T> {
            let start_idx = row_idx * self.columns;
            let stop_idx = start_idx + self.columns;
            RowIterMut(self.elements[start_idx..stop_idx].iter_mut())
        }

        pub fn column(&self, col_idx: usize) -> ColumnIter<T> {
            ColumnIter {
                row_idx: 0,
                col_idx,
                matrix: &self
            }
        }

        // pub fn column_mut(&self, col_idx: usize) -> ColumnIterMut<T> {
        //     ColumnIterMut {
        //         row_idx: 0,
        //         col_idx,
        //         matrix: &mut self
        //     }
        // }
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