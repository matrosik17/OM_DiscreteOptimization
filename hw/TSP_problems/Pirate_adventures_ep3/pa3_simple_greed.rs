use std::io::{BufRead};
use std::collections::HashMap;

type Island = (i64, i64, i64);

fn dist(isl1: &Island, isl2: &Island) -> i64 {
    let (x1, y1, _) = isl1;
    let (x2, y2, _) = isl2;
    let d2 = (x2 - x1).pow(2) + (y2 - y1).pow(2);
    (d2 as f64).sqrt().round() as i64
}

fn cost(p: i64, isl1: &Island, isl2: &Island) -> i64 {
    let d = dist(isl1, isl2);
    isl2.2 - p * d
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let p: i64 = scan.token();
    let _k: usize = scan.token();
    let _m: i64 = scan.token();

    let home_idx = 0;
    let home: Island = (scan.token(), scan.token(), scan.token());
    let mut current_position: Island = home;

    let mut islands: HashMap<usize, Island> = HashMap::with_capacity(n);
    for idx in 1..n {
        let island = (scan.token(), scan.token(), scan.token());
        islands.insert(idx, island);
    }

    let mut path = vec![home_idx];
    while !islands.is_empty() {
        let (best_neigh_idx, island) = islands.iter().max_by(|(_, isl1), (_, isl2)| {
            let cost1 = cost(p, &current_position, isl1);
            let cost2 = cost(p, &current_position, isl2);
            cost1.cmp(&cost2)
        }).unwrap();

        // if cost(p, &current_position, &island) < 0 {
        //     if current_position == home { break; }
        //     else {
        //         path.push(home_idx);
        //         current_position = home;
        //     }
        // } else {
        //     path.push(best_neigh_idx);
        //     current_position = island;
        // }
        if cost(p, &current_position, island) < 0 { break; }
        else {
            let best_neigh_idx = *best_neigh_idx;
            let island = islands.remove(&best_neigh_idx).unwrap();
            path.push(best_neigh_idx);
            current_position = island;
        }
    }

    path.push(home_idx);
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
