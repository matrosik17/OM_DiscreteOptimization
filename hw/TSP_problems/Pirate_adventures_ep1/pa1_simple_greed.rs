use std::io::{BufRead};
use std::collections::{HashMap, VecDeque};

type Island = (i64, i64, i64);

struct TravelInfo {
    dist_cost: i64,
    max_len: usize,
    pub max_score: i64,
    prev_path: VecDeque<i64>,
}

impl TravelInfo {
    fn new(p: i64, k: usize, m: i64) -> TravelInfo {
        TravelInfo {
            dist_cost: p,
            max_len: k,
            max_score: m,
            prev_path: VecDeque::with_capacity(k),
        }
    }

    fn path_cost(&self, next_cost: i64) -> i64 {
        let prev_path_cost: i64 = self.prev_path.iter().rev().take(self.max_len - 1).sum();
        prev_path_cost + next_cost
    }

    fn update(&mut self, island_cost: i64) {
        if self.prev_path.len() == self.max_len {
            self.prev_path.pop_front();
        }
        self.prev_path.push_back(island_cost);
    }

    fn cost(&self, isl1: &Island, isl2: &Island) -> i64 {
        let d = dist(isl1, isl2);
        let score = isl2.2 - self.dist_cost * d;
        score
    }
}


fn dist(isl1: &Island, isl2: &Island) -> i64 {
    let (x1, y1, _) = isl1;
    let (x2, y2, _) = isl2;
    let d2 = (x2 - x1).pow(2) + (y2 - y1).pow(2);
    (d2 as f64).sqrt().round() as i64
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let p: i64 = scan.token();
    let k: usize = scan.token();
    let m: i64 = scan.token();

    let mut islands: HashMap<usize, Island> = HashMap::with_capacity(n);
    let mut attended_islands: HashMap<usize, Island> = HashMap::with_capacity(n);

    let home_idx = 0;
    let home: Island = (scan.token(), scan.token(), scan.token());

    attended_islands.insert(home_idx, home);
    let mut current_position: Island = home;

    let mut travel_info = TravelInfo::new(p, k, m);

    for idx in 1..n {
        let island = (scan.token(), scan.token(), scan.token());
        islands.insert(idx, island);
    }

    // greedy path finder
    let mut path = vec![home_idx];
    while !islands.is_empty() {
        let (best_neigh_idx, island) = islands.iter().max_by(|(_, isl1), (_, isl2)| {
            let cost1 = travel_info.cost(&current_position, isl1);
            let cost2 = travel_info.cost(&current_position, isl2);
            cost1.cmp(&cost2)
        }).unwrap();

        let island_cost = travel_info.cost(&current_position, island);
        if island_cost < 0 { break; }
        else {
            let best_neigh_idx = *best_neigh_idx;
            let island = islands.remove(&best_neigh_idx).unwrap();
            attended_islands.insert(best_neigh_idx, island);

            path.push(best_neigh_idx);
            current_position = island;
        }
    }
    path.push(home_idx);

    // fix feasibility
    let path: Vec<usize> = path.into_iter()
        .filter(|isl_idx| {
            let (_, _, cost) = attended_islands.get(isl_idx).unwrap();
            let path_cost = travel_info.path_cost(*cost);
            if path_cost >= travel_info.max_score { false }
            else {
                travel_info.update(*cost);
                true
            }
        })
        .collect();

    let result = path.into_iter()
        .map(|x| x + 1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    print!("{}", result);
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
