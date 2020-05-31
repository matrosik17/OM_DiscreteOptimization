use io::Scanner;
use rng::Xoshiro256ss;
use std::cmp::Ordering;
use std::io::Write;
use std::time::{Instant, Duration};

type Point = (i64, f64, f64);
type Node = i64;
type Edge = f64;
type DMatrix = matrix::Matrix<Edge>;
type Graph = graph::Graph<Node, Edge>;

fn distance(p1: &Point, p2: &Point) -> Edge {
    let (_, x1, y1) = p1;
    let (_, x2, y2) = p2;
    ((x2 - x1).powf(2.) + (y2 - y1).powf(2.)).sqrt()
}

fn build_dist_matrix(points: &[Point]) -> DMatrix {
    let n_nodes = points.len();
    let mut matrix = DMatrix::empty_squared(n_nodes);
    for i in 0..n_nodes {
        for j in 0..n_nodes {
            matrix[(i, j)] = distance(&points[i], &points[j]);
        }
    }
    matrix
}

fn build_mst(dist_matrix: &DMatrix) -> Vec<(usize, usize, Edge)> {
    let (n_nodes, _) = dist_matrix.shape();
    let mut in_mst = vec![false; n_nodes];

    in_mst[0] = true;
    let mut edge_ends = vec![0_usize; n_nodes];
    let mut edge_weights: Vec<Edge> = (0..n_nodes)
        .map(|node_idx| dist_matrix[(0, node_idx)])
        // .map(|node_idx| distance(&points[0], &points[node_idx]))
        .collect();

    for _ in 1..n_nodes {
        let (new_mst_node_idx, _) = edge_weights.iter()
            .enumerate()
            .filter(|(node_idx, _)| !in_mst[*node_idx])
            .min_by(|(_, w1), (_, w2)| match w1.partial_cmp(w2) {
                Some(ord) => ord,
                None => Ordering::Less,
            })
            .unwrap();

        in_mst[new_mst_node_idx] = true;
        for j in 0..n_nodes {
            if in_mst[j] { continue; }
            else {
                // let weight = distance(&points[j], &points[new_mst_node_idx]);
                let weight = dist_matrix[(j, new_mst_node_idx)];
                if weight < edge_weights[j] {
                    edge_weights[j] = weight;
                    edge_ends[j] = new_mst_node_idx;
                }
            }
        }
    }

    let edges = edge_ends.into_iter()
        .enumerate()
        .filter_map(|edge| match edge {
            (i, j) if i == j => None,
            (i, j) => Some((i, j, edge_weights[i])),
        })
        .collect();

    edges
}

fn find_euler_cycle(mut graph: Graph) -> Vec<usize> {
    let (n_nodes, _) = graph.size();
    let mut stack = Vec::with_capacity(n_nodes);
    let mut path = Vec::with_capacity(n_nodes);

    let start_node = 0_usize;
    stack.push(start_node);

    while !stack.is_empty() {
        let &node_idx = stack.last().unwrap();
        if graph.neigh(node_idx).len() == 0 {
            let path_node = stack.pop().unwrap();
            path.push(path_node);
        } else {
            let (next_node, _) = graph.neigh_mut(node_idx).pop().unwrap();
            stack.push(next_node);
        }
    }
    path
}

fn relax_euler_cycle(path: Vec<usize>) -> Vec<usize> {
    let n_nodes = path.iter().max().unwrap() + 1;
    let mut included = vec![false; n_nodes];

    let mut hamilton_cycle: Vec<usize> = path.into_iter()
        .filter(|&node| {
            if !included[node] {
                included[node] = true;
                true
            } else {
                false
            }
        })
        .collect();
    let start_node = *hamilton_cycle.first().unwrap();
    hamilton_cycle.push(start_node);
    hamilton_cycle
}

fn calc_cycle_weight(cycle: &[usize], dist_matrix: &DMatrix) -> f64 {
    let mut weight = 0.;
    let mut prev_node = cycle[0];
    for node in cycle.iter().skip(1) {
        weight += dist_matrix[(prev_node, *node)];
        prev_node = *node;
    }
    weight += dist_matrix[(prev_node, cycle[0])];
    weight
}

// const TIME_LIMIT: u64 = 30_000;
const TIME_LIMIT: u64 = 5_000;
const STOP_TIME: Duration = Duration::from_millis(TIME_LIMIT - 1);

fn local_search_2opt(cycle: Vec<usize>, dist_matrix: &DMatrix, rng: &mut Xoshiro256ss) -> Vec<usize> {
    let path_len = cycle.len();
    let curr_weight = calc_cycle_weight(&cycle, dist_matrix);

    let mut cycle_copy = cycle.clone();
    let mut split_nodes = [
        (rng.rand() as usize % (path_len - 2)) + 1,
        (rng.rand() as usize % (path_len - 2)) + 1
    ];
    split_nodes.sort();
    let [node1, node2] = split_nodes;

    cycle_copy[node1..node2].reverse();
    let shuffle_weight = calc_cycle_weight(&cycle_copy, dist_matrix);

    if shuffle_weight < curr_weight {
        // println!("2-opt improvement: {} -> {}", curr_weight, shuffle_weight);
        cycle_copy
    } else {
        cycle
    }
}

fn local_search_3opt(cycle: Vec<usize>, dist_matrix: &DMatrix, rng: &mut Xoshiro256ss) -> Vec<usize> {
    let path_len = cycle.len();
    let curr_weight = calc_cycle_weight(&cycle, dist_matrix);

    let mut cycle_copy = cycle.clone();
    let mut split_nodes =[
        (rng.rand() as usize % (path_len - 2)) + 1,
        (rng.rand() as usize % (path_len - 2)) + 1,
        (rng.rand() as usize % (path_len - 2)) + 1
    ];
    split_nodes.sort();
    let [node1, node2, node3] = split_nodes;

    // граничные точки сегментов
    let A = cycle[node1 - 1];
    let B = cycle[node1];
    let C = cycle[node2 - 1];
    let D = cycle[node2];
    let E = cycle[node3 - 1];
    let F = cycle[node3];

    // различные способы соединить эти сегменты
    let d0 = dist_matrix[(A, B)] + dist_matrix[(C, D)] + dist_matrix[(E, F)];
    let d1 = dist_matrix[(A, C)] + dist_matrix[(B, D)] + dist_matrix[(E, F)];
    let d2 = dist_matrix[(A, B)] + dist_matrix[(C, E)] + dist_matrix[(D, F)];
    let d3 = dist_matrix[(A, D)] + dist_matrix[(E, B)] + dist_matrix[(C, F)];
    let d4 = dist_matrix[(F, B)] + dist_matrix[(C, D)] + dist_matrix[(E, A)];

    if d0 > d1 {
        cycle_copy[node1..node2].reverse();
        // let shuffle_weight = calc_cycle_weight(&cycle_copy, dist_matrix);
        // if shuffle_weight < curr_weight {
        //     println!("3-opt improvement: {} -> {}", curr_weight, shuffle_weight);
        // }
        cycle_copy
    } else if d0 > d2 {
        cycle_copy[node2..node3].reverse();
        // let shuffle_weight = calc_cycle_weight(&cycle_copy, dist_matrix);
        // if shuffle_weight < curr_weight {
        //     println!("3-opt improvement: {} -> {}", curr_weight, shuffle_weight);
        // }
        cycle_copy
    } else if d0 > d4 {
        cycle_copy[node1..node3].reverse();
        // let shuffle_weight = calc_cycle_weight(&cycle_copy, dist_matrix);
        // if shuffle_weight < curr_weight {
        //     println!("3-opt improvement: {} -> {}", curr_weight, shuffle_weight);
        // }
        cycle_copy
    } else if d0 > d3 {
        let mid = node3 - node2;
        cycle_copy[node1..node3].rotate_left(mid);
        // let shuffle_weight = calc_cycle_weight(&cycle_copy, dist_matrix);
        // if shuffle_weight < curr_weight {
        //     println!("3-opt improvement: {} -> {}", curr_weight, shuffle_weight);
        // }
        cycle_copy
    } else {
        cycle
    }


    // // println!("{:?}", (node1, node2));
    // cycle_copy.swap(node1, node2);
    // cycle_copy.swap(node1, node3);
    // let shuffle_weight = calc_cycle_weight(&cycle_copy, dist_matrix);

    // if shuffle_weight < curr_weight {
    //     println!("3-opt improvement: {} -> {}", curr_weight, shuffle_weight);
    //     cycle_copy
    // } else {
    //     cycle
    // }
}

fn local_search(cycle: Vec<usize>, dist_matrix: &DMatrix, rng: &mut Xoshiro256ss) -> Vec<usize> {
    match rng.rand() % 2 {
        0 => local_search_2opt(cycle, dist_matrix, rng),
        1 => local_search_2opt(cycle, dist_matrix, rng),
        _ => unreachable!(),
    }
}

fn main() {
    let start_time = Instant::now();

    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());

    let n: usize = scan.token();
    let points: Vec<Point> = (0..n)
        .map(|_| (scan.token(), scan.token(), scan.token()))
        .collect();

    let dist_matrix = build_dist_matrix(&points);
    let mst_edges = build_mst(&dist_matrix);

    let nodes = points.iter().map(|x| x.0).collect();
    let mut mst_graph = Graph::from_nodes(nodes);
    for (node1, node2, weight) in mst_edges {
        mst_graph.add_edge_undirected(node1, node2, weight);
    }

    let euler_cycle = find_euler_cycle(mst_graph);
    let mut hamilton_cycle = relax_euler_cycle(euler_cycle);

    // начинаем улучшать решение
    let seed: u64 = 42;
    let mut rng = Xoshiro256ss::new(seed);
    loop {
        let duration = start_time.elapsed();
        if duration < STOP_TIME {
            hamilton_cycle = local_search(hamilton_cycle, &dist_matrix, &mut rng);
        } else {
            break;
        }
    }

    // println!("cycle weight: {}", calc_cycle_weight(&hamilton_cycle, &dist_matrix));

    for node_idx in hamilton_cycle {
        let (node_id, _, _) = points[node_idx];
        let _ = write!(writer, "{} ", node_id);
    }
    let _ = write!(writer, "\n");
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

mod rng {

    pub struct Xoshiro256ss {
        state: [u64; 4],
    }

    impl Xoshiro256ss {
        pub fn new(seed: u64) -> Self {
            let mut state = [0; 4];
            let mut split_mix = SplitMix64 { state: seed };
            state[0] = split_mix.rand();
            state[1] = split_mix.rand();
            state[2] = split_mix.rand();
            state[3] = split_mix.rand();
            Self { state }
        }

        fn rol64(x: u64, k: i64) ->u64 {
            (x << k) | (x >> (64 - k))
        }

        pub fn rand(&mut self) -> u64 {
            let result = Self::rol64(self.state[1].wrapping_mul(5), 7).wrapping_mul(9);
            let t = self.state[1] << 17;

            self.state[2] ^= self.state[0];
            self.state[3] ^= self.state[1];
            self.state[1] ^= self.state[2];
            self.state[0] ^= self.state[3];

            self.state[2] ^= t;
            self.state[3] = Self::rol64(self.state[3], 45);

            return result;
        }
    }

    struct SplitMix64 {
        state: u64,
    }

    impl SplitMix64 {
        fn rand(&mut self) -> u64 {
            let mut result = self.state;
            self.state = result.wrapping_add(0x9E3779B97f4A7C15);
            result = (result ^ (result >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            result = (result ^ (result >> 27)).wrapping_mul(0x94D049BB133111EB);
            return result ^ (result >> 31);
        }
    }
}