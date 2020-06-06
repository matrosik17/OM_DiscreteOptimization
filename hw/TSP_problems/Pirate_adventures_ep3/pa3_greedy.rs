use io::Scanner;
use std::cmp::Ordering;

type Island = (f64, f64,f64);
type DMatrix = matrix::Matrix<f64>;

fn distance(isl1: &Island, isl2: &Island) -> f64 {
    let (x1, y1, _) = isl1;
    let (x2, y2, _) = isl2;
    let d2 = (x2 - x1).powf(2.) + (y2 - y1).powf(2.);
    d2.sqrt()
}

fn build_dist_matrix(islands: &[Island]) -> DMatrix {
    let n_nodes = islands.len();
    let mut matrix = DMatrix::empty_squared(n_nodes);
    for i in 0..n_nodes {
        for j in 0..n_nodes {
            matrix[(i, j)] = distance(&islands[i], &islands[j]);
        }
    }
    matrix
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let p: f64 = scan.token();
    let _k: usize = scan.token();
    let _m: f64 = scan.token();

    let islands: Vec<Island> = (0..n).map(|_| (scan.token(), scan.token(), scan.token())).collect();
    let dist_matrix = build_dist_matrix(&islands);

    let mut used = vec![false; n];
    let mut path = Vec::with_capacity(n);

    let home_idx: usize = 0;
    path.push(home_idx);
    used[home_idx] = true;
    let mut curr_idx = home_idx;

    // жадно ищем путь
    while !used.iter().all(|&x| x == true) {
        let (next_idx, next_island, profit_rate) = islands.iter()
            .enumerate()
            .filter(|(idx, _)| *idx != curr_idx && !used[*idx])
            .map(|(idx, isl)| {
                let dist = dist_matrix[(curr_idx, idx)];
                let travel_cost = p * dist;
                let (_, _, reward) = isl;
                let profit_rate = (reward - travel_cost) / dist;
                (idx, isl, profit_rate)
            })
            .max_by(|(_, _, pr1), (_, _, pr2)| match pr1.partial_cmp(pr2) {
                Some(ord) => ord,
                None => Ordering::Less,
            })
            .unwrap();

        if profit_rate < 0. { break; }
        else {
            path.push(next_idx);
            used[next_idx] = true;
            curr_idx = next_idx;
        }
    }
    // возвращаемся на базу
    path.push(home_idx);

    // выводим полученный маршрут
    let result = path.into_iter()
        .map(|x| x + 1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
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
