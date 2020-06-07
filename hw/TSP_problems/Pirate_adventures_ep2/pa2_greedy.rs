use io::Scanner;
use rng::Xoshiro256ss;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

type DMatrix = matrix::Matrix<f64>;
type Queue = VecDeque<f64>;

struct PAProblem {
    pub p: f64,
    pub k: usize,
    pub max_sum: f64,
    pub dist_matrix: DMatrix,
    pub rewards: Vec<f64>,
}

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

fn greedy_search(pa_problem: &PAProblem) -> Vec<usize> {
    let n = pa_problem.rewards.len();
    let mut used = vec![false; n];
    let mut path = Vec::with_capacity(n);
    let mut queue = Queue::with_capacity(pa_problem.k);
    let mut curr_reward = 0.;

    let home_idx: usize = 0;
    path.push(home_idx);
    used[home_idx] = true;
    let mut curr_idx = home_idx;

    // жадно ищем путь
    while !used.iter().all(|&x| x == true) {
        let best_next_island = pa_problem.rewards.iter()
            .enumerate()
            .filter(|(idx, &reward)| {
                if *idx == curr_idx { false }
                else if used[*idx] { false }
                else {
                    let sum: f64 = queue.iter().skip(1).sum();
                    sum + reward < pa_problem.max_sum
                }
            })
            .map(|(idx, &reward)| {
                let dist = pa_problem.dist_matrix[(curr_idx, idx)];
                let travel_cost = pa_problem.p * dist;
                let income_reward = reward - travel_cost;
                let profit_rate = income_reward / dist.powf(0.8);
                (idx, income_reward, profit_rate)
            })
            .max_by(|(_, _, pr1), (_, _, pr2)| match pr1.partial_cmp(pr2) {
                Some(ord) => ord,
                None => Ordering::Greater,
            });

        if let Some((next_idx, income_reward, _)) = best_next_island {
            if income_reward < 0. {
                let lose_rate = income_reward.abs() / curr_reward;
                let free_money: f64 = (0..n)
                    .filter(|idx| !used[*idx])
                    .map(|idx| pa_problem.rewards[idx])
                    .sum();

                if lose_rate > 0.1 { break; }
                if free_money + income_reward < 0. {
                    break;
                }
            }

            path.push(next_idx);
            used[next_idx] = true;
            curr_idx = next_idx;

            curr_reward += income_reward;
            queue.push_back(pa_problem.rewards[curr_idx]);
            if queue.len() > pa_problem.k { queue.pop_front(); }
        } else {
            break;
        }

    }
    // возвращаемся на базу
    path.push(home_idx);

    path
}

fn is_correct_path(path: &[usize], pa_problem: &PAProblem) -> bool {
    let k = pa_problem.k;
    let max_sum = pa_problem.max_sum;
    let rewards = &pa_problem.rewards;

    for window in path.windows(k) {
        let sum: f64 = window.iter().map(|&idx| rewards[idx]).sum();
        if sum > max_sum { return false; }
    }
    true
}

fn total_reward(path: &[usize], pa_problem: &PAProblem) -> f64 {
    let mut total_reward = 0.;
    let mut prev_isl: usize = 0;
    for &isl_idx in path.iter().skip(1) {
        let isl_reward = pa_problem.rewards[isl_idx];
        let travel_cost = pa_problem.p * pa_problem.dist_matrix[(prev_isl, isl_idx)];
        total_reward += isl_reward - travel_cost;
        prev_isl = isl_idx;
    }
    total_reward
}

fn local_search_3opt(mut path: Vec<usize>, pa_problem: &PAProblem, split: [usize; 3]) -> (f64, Vec<usize>) {
    let dist_matrix = &pa_problem.dist_matrix;
    let [node1, node2, node3] = split;

    // граничные точки сегментов
    let a = path[node1 - 1];
    let b = path[node1];
    let c = path[node2 - 1];
    let d = path[node2];
    let e = path[node3 - 1];
    let f = path[node3];

    // различные способы соединить эти сегменты
    let d0 = dist_matrix[(a, b)] + dist_matrix[(c, d)] + dist_matrix[(e, f)];
    let d1 = dist_matrix[(a, c)] + dist_matrix[(b, d)] + dist_matrix[(e, f)];
    let d2 = dist_matrix[(a, b)] + dist_matrix[(c, e)] + dist_matrix[(d, f)];
    let d3 = dist_matrix[(a, d)] + dist_matrix[(e, b)] + dist_matrix[(c, f)];
    let d4 = dist_matrix[(f, b)] + dist_matrix[(c, d)] + dist_matrix[(e, a)];

    if d0 > d1 {
        path[node1..node2].reverse();
        (d1 - d0, path)
    } else if d0 > d2 {
        path[node2..node3].reverse();
        (d2 - d0, path)
    } else if d0 > d4 {
        path[node1..node3].reverse();
        (d4 - d0, path)
    } else if d0 > d3 {
        let mid = node3 - node2;
        path[node1..node3].rotate_left(mid);
        (d3 - d0, path)
    } else {
        (0., path)
    }
}

fn local_search_add(mut path: Vec<usize>, pa_problem: &PAProblem) -> Vec<usize> {
    let p = pa_problem.p;
    let dist_matrix = &pa_problem.dist_matrix;
    let rewards = &pa_problem.rewards;

    let n = rewards.len();
    let mut used = vec![false; n];
    path.iter().for_each(|&idx| used[idx] = true);

    while used.iter().any(|is_used| !is_used) {
        let (curr_idx, _) = used.iter().enumerate().find(|(_, &is_used)| !is_used).unwrap();
        used[curr_idx] = true;

        let (path_idx, income_reward) = path.windows(2)
            .enumerate()
            .map(|(path_idx, path_pair)| {
                let (isl1_idx, isl2_idx) = match path_pair {
                    [idx1, idx2] => (idx1, idx2),
                    _ => unreachable!(),
                };
                let travel_cost_to = p * dist_matrix[(*isl1_idx, curr_idx)];
                let travel_cost_from = p * dist_matrix[(curr_idx, *isl2_idx)];
                let income_reward = rewards[curr_idx] + rewards[*isl2_idx] - travel_cost_to - travel_cost_from;
                (path_idx, income_reward)
            })
            .max_by(|(_, ir1), (_, ir2)| match ir1.partial_cmp(&ir2) {
                Some(ord) => ord,
                None => Ordering::Greater,
            })
            .unwrap();

        // проверяем kM-условие
        if income_reward > 0. {
            path.insert(path_idx + 1, curr_idx);
            if !is_correct_path(&path, pa_problem) {
                path.remove(path_idx + 1);
            }
        }
    }
    path
}

const TIME: Duration = Duration::from_secs(10 * 60);

fn local_search(mut path: Vec<usize>, pa_problem: &PAProblem) -> Vec<usize> {
    path = local_search_add(path, pa_problem);

    let seed: u64 = 42;
    let mut rng = Xoshiro256ss::new(seed);

    let path_len = path.len();
    let start_time = Instant::now();
    loop {
        if start_time.elapsed() < TIME {
            let mut split = [
                (rng.rand() as usize % (path_len - 2)) + 1,
                (rng.rand() as usize % (path_len - 2)) + 1,
                (rng.rand() as usize % (path_len - 2)) + 1
            ];
            split.sort();

            let (_, path_mut) = local_search_3opt(path.clone(), pa_problem, split);

            if is_correct_path(&path_mut, pa_problem) {
                path = path_mut;
            }
        } else {
            break;
        }
    }
    path
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

    let dist_matrix = build_dist_matrix(&coordinates);

    let pa_problem = PAProblem { p, k, max_sum, dist_matrix, rewards };
    let max_reward: f64 = pa_problem.rewards.iter().sum();

    // поиск решения
    let mut path = greedy_search(&pa_problem);
    path = local_search(path, &pa_problem);

    let total_reward = total_reward(&path, &pa_problem);

    println!("path is correct: {}", is_correct_path(&path, &pa_problem));
    println!("{}", max_reward);
    println!("{}", total_reward);

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
        columns: usize,
        pub elements: Vec<T>,
    }

    impl<T: Default + Clone> Matrix<T> {
        pub fn new(_rows: usize, columns: usize, elements: Vec<T>) -> Self {
            Self { columns, elements }
        }

        pub fn empty(rows: usize, columns: usize) -> Self {
            let elements = vec![T::default(); rows * columns];
            Self::new(rows, columns, elements)
        }

        pub fn empty_squared(size: usize) -> Self {
            Self::empty(size, size)
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
