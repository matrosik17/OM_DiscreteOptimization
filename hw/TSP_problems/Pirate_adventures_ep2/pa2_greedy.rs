use io::Scanner;
use std::cmp::Ordering;
use std::collections::VecDeque;

type DMatrix = matrix::Matrix<f64>;
type Queue = VecDeque<f64>;

fn distance(coord1: &(f64, f64), coord2: &(f64, f64)) -> f64 {
    let (x1, y1) = coord1;
    let (x2, y2) = coord2;
    let d2 = (x2 - x1).powf(2.) + (y2 - y1).powf(2.);
    d2.sqrt()
}

fn build_dist_matrix(coordinates: &[(f64, f64)]) -> DMatrix {
    let n_nodes = coordinates.len();
    let mut matrix = DMatrix::empty_squared(n_nodes);
    for i in 0..n_nodes {
        for j in 0..n_nodes {
            matrix[(i, j)] = distance(&coordinates[i], &coordinates[j]);
        }
    }
    matrix
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let p: f64 = scan.token();
    let k: usize = scan.token();
    let max_sum: f64 = scan.token();

    let mut coordinates: Vec<(f64, f64)> = Vec::with_capacity(n);
    let mut rewards: Vec<f64> = Vec::with_capacity(n);

    for _ in 0..n {
        coordinates.push((scan.token(), scan.token()));
        rewards.push(scan.token());
    }

    let max_reward: f64 = rewards.iter().sum();
    let dist_matrix = build_dist_matrix(&coordinates);

    let mut used = vec![false; n];
    let mut path = Vec::with_capacity(n);
    let mut queue = Queue::with_capacity(k);
    let mut curr_reward = 0.;

    let home_idx: usize = 0;
    path.push(home_idx);
    used[home_idx] = true;
    let mut curr_idx = home_idx;

    // жадно ищем путь
    while !used.iter().all(|&x| x == true) {
        let best_next_island = rewards.iter()
            .enumerate()
            .filter(|(idx, &reward)| {
                if *idx == curr_idx { false }
                else if used[*idx] { false }
                else {
                    let sum: f64 = queue.iter().skip(1).sum();
                    sum + reward < max_sum
                }
            })
            .map(|(idx, &reward)| {
                let dist = dist_matrix[(curr_idx, idx)];
                let travel_cost = p * dist;
                let income_reward = reward - travel_cost;
                let profit_rate = income_reward / dist.powf(0.8);
                (idx, income_reward, profit_rate)
            })
            .max_by(|(_, _, pr1), (_, _, pr2)| match pr1.partial_cmp(pr2) {
                Some(ord) => ord,
                None => Ordering::Less,
            });

        if let Some((next_idx, income_reward, profit_rate)) = best_next_island {
            if income_reward < 0. {
                let lose_rate = income_reward.abs() / curr_reward;
                let free_money: f64 = (0..n)
                    .filter(|idx| !used[*idx])
                    .map(|idx| rewards[idx])
                    .sum();

                if lose_rate > 0.1 { break; }
                if free_money + income_reward < 0. { break; }
            }

            path.push(next_idx);
            used[next_idx] = true;
            curr_idx = next_idx;

            curr_reward += income_reward;
            queue.push_back(income_reward);
            if queue.len() > k { queue.pop_front(); }
        } else {
            break;
        }

    }
    // возвращаемся на базу
    path.push(home_idx);

    // println!("{}", max_reward);
    // println!("{}", curr_reward);

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
        pub elements: Vec<T>,
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
