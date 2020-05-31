use io::Scanner;
use rng::Xoshiro256ss;
use std::cmp::Ordering;
use std::io::Write;
use std::time::{Instant, Duration};

type Point = (i64, f64, f64);
type Node = i64;
type Edge = f64;
type DMatrix = matrix::Matrix<Edge>;

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

fn find_cycle_greedy(dist_matrix: &DMatrix) -> Vec<usize> {
    let (n_nodes, _) = dist_matrix.shape();
    let mut included = vec![false; n_nodes];
    let mut cycle = Vec::with_capacity(n_nodes + 1);

    let mut curr_node = 0_usize;
    included[curr_node] = true;
    cycle.push(curr_node);

    while !included.iter().all(|&x| x) {
        let (next_node, _) = included.iter()
            .enumerate()
            .filter(|(_, &x)| !x)
            .map(|(node, _)| (node, dist_matrix[(curr_node, node)]))
            .min_by(|(_, w1), (_, w2)| match w1.partial_cmp(&w2) {
                Some(ord) => ord,
                None => Ordering::Less,
            }).unwrap();
        included[next_node] = true;
        cycle.push(next_node);
        curr_node = next_node;
    }
    cycle.push(0);
    cycle
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
const TIME_LIMIT: u64 = 10_000;
const STOP_TIME: Duration = Duration::from_millis(TIME_LIMIT - 1);
const MUTATION_RATE: f64 = 1.0;

// fn local_search_2opt(mut cycle: Vec<usize>, _dist_matrix: &DMatrix, rng: &mut Xoshiro256ss) -> Vec<usize> {
//     let path_len = cycle.len();
//     let mut split_nodes = [
//         (rng.rand() as usize % (path_len - 2)) + 1,
//         (rng.rand() as usize % (path_len - 2)) + 1
//     ];
//     split_nodes.sort();
//     let [node1, node2] = split_nodes;

//     cycle[node1..node2].reverse();
//     cycle
// }

fn local_search_3opt(mut cycle: Vec<usize>, dist_matrix: &DMatrix, split: [usize; 3]) -> (f64, Vec<usize>) {
    let [node1, node2, node3] = split;

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
        cycle[node1..node2].reverse();
        (d1 - d0, cycle)
    } else if d0 > d2 {
        cycle[node2..node3].reverse();
        (d2 - d0, cycle)
    } else if d0 > d4 {
        cycle[node1..node3].reverse();
        (d4 - d0, cycle)
    } else if d0 > d3 {
        let mid = node3 - node2;
        cycle[node1..node3].rotate_left(mid);
        (d3 - d0, cycle)
    } else {
        (0., cycle)
    }
}

fn local_search(mut cycle: Vec<usize>, dist_matrix: &DMatrix, _rng: &mut Xoshiro256ss) -> Vec<usize> {
    // let mut cycle_clone = cycle.clone();
    let path_len = cycle.len();
    // let mut split = [
    //     (rng.rand() as usize % (path_len - 2)) + 1,
    //     (rng.rand() as usize % (path_len - 2)) + 1,
    //     (rng.rand() as usize % (path_len - 2)) + 1
    // ];
    // split.sort();

    let mut total_diff = 0.;
    for node1 in 1..path_len {
        for node2 in (node1 + 2)..path_len {
            for node3 in (node2 + 2)..path_len {
                let split = [node1, node2, node3];
                let (diff, cycle_mut) = local_search_3opt(cycle, dist_matrix, split);
                cycle = cycle_mut;
                total_diff += diff;
            }
        }
    }
    cycle
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
    let mut hamilton_cycle = find_cycle_greedy(&dist_matrix);

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

    println!("cycle weight: {}", calc_cycle_weight(&hamilton_cycle, &dist_matrix));
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