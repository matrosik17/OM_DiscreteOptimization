use io::Scanner;
use std::ops::Index;

type Point = (i64, f64, f64);

fn distance(p1: &Point, p2: &Point) -> f64 {
    let (_, x1, y1) = p1;
    let (_, x2, y2) = p2;
    ((x2 - x1).powf(2.) + (y2 - y1).powf(2.)).sqrt()
}

struct FullGraph {
    n_nodes: usize,
    edges: Vec<f64>, // вытянутый 2d массив. edges[(i, j)] - расстояние от точки i до точки j
}

impl FullGraph {
    pub fn new(points: &[Point]) -> Self {
        let n_nodes = points.len();

        let mut edges: Vec<f64> = Vec::with_capacity(n_nodes * n_nodes);
        for p1 in points.iter() {
            for p2 in points.iter() {
                let weight = distance(p1, p2);
                edges.push(weight);
            }
        }

        Self { n_nodes, edges }
    }
}

// graph[(i,j)] - вес ребра ij
impl Index<(usize, usize)> for FullGraph {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        let edge_idx = i * self.n_nodes + j;
        &self.edges[edge_idx]
    }
}

// graph[i] - все соседи вершины i (а также сам i)
impl Index<usize> for FullGraph {
    type Output = [f64];

    fn index(&self, node_idx: usize) -> &Self::Output {
        let start_idx = node_idx * self.n_nodes;
        let stop_idx = start_idx + self.n_nodes;
        &self.edges[start_idx..stop_idx]
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let points: Vec<(i64, f64, f64)> = (0..n)
        .map(|_| (scan.token(), scan.token(), scan.token()))
        .collect();

    let graph = FullGraph::new(&points);
    println!("{:?}", graph.edges);

    // println!("{:?}", points);
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