use std::io::{BufRead};
use std::ops::{Index, IndexMut};

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let p: i64 = scan.token();
    let _k: usize = scan.token();
    let _M: i64 = scan.token();

    let mut islands: Vec<(i64, i64, i64)> = Vec::with_capacity(n);
    for _ in 0..n {
        islands.push((scan.token(), scan.token(), scan.token()));
    }

    let mut adj_matrix = Matrix::new(n, n);
    for i in 0..n {
        for j in 0..n {
            let (x1, y1, m1) = islands[i];
            let (x2, y2, m2) = islands[j];

            let d2 = (x2 - x1).pow(2) + (y2 - y1).pow(2);
            let d = (d2 as f64).sqrt().round() as i64;
            let move_cost = p * d;
            let weight = m2 - move_cost;
            adj_matrix[(i, j)] = weight;
        }
    }

    // Floyd-Warshall maximize
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if i == k { continue; }
                adj_matrix[(i, j)] = std::cmp::max(
                    adj_matrix[(i, j)],
                    adj_matrix[(i, k)] + adj_matrix[(k, j)]
                );
            }
        }
        println!("{}", adj_matrix[(0, 0)]);
    }
    // println!("{:?}", islands);
}

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

struct Matrix {
    rows: usize,
    columns: usize,
    field: Vec<i64>,
}

impl Matrix {
    pub fn new(n: usize, m: usize) -> Matrix {
        let size = n * m;
        Matrix {
            rows: n,
            columns: m,
            field: vec![0; size]
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = i64;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (n, m) = idx;
        let idx = n * self.columns + m;
        &self.field[idx]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (n, m) = idx;
        let idx = n * self.columns + m;
        &mut self.field[idx]
    }
}
