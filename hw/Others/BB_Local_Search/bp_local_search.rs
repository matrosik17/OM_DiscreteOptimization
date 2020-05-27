use io::Scanner;
use std::fmt;
use std::fmt::Display;
use std::time::Instant;

struct BBProblem {
    pub capacity: usize,
    pub weights: Vec<usize>,
}

#[derive(Clone)]
struct Packing {
    pub indices: Vec<usize>,
}

impl Packing {
    // pub fn len(&self) -> usize {
    //     self.indices.len()
    // }

    // pub fn put_item(&mut self, bin_idx: usize) {
    //     self.indices.push(bin_idx);
    // }

    // pub fn truncate(&mut self, len: usize) {
    //     self.indices.truncate(len);
    // }

    pub fn num_bins(&self) -> usize {
        *self.indices.iter().max().unwrap_or(&0) + 1
    }

    // pub fn is_correct_packing(&self, problem: &BBProblem) -> bool {
    //     let mut bins = vec![0_usize; self.num_bins()];
    //     for (item_idx, bin_idx) in self.indices.iter().enumerate() {
    //         let bin_weight = &mut bins[*bin_idx];
    //         *bin_weight += problem.weights[item_idx];
    //         if *bin_weight > problem.capacity { return false; }
    //     }
    //     true
    // }
}

impl Display for Packing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result_str = self.indices.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", result_str)
    }
}

fn first_fit(problem: &BBProblem) -> Packing {
    let mut indices: Vec<usize> = vec![std::usize::MAX; problem.weights.len()];
    let mut bin_spaces: Vec<usize> = Vec::with_capacity(indices.len());

    for (item_idx, weight) in problem.weights.iter().enumerate() {
        let bin = {
                bin_spaces.iter_mut()
                .enumerate()
                .find_map(|(bin_idx, x)| {
                    if *x > *weight {
                        *x -= *weight;
                        indices[item_idx] = bin_idx;
                        Some(*x)
                    } else {
                        None
                    }
                })
            };

        if bin == None {
            bin_spaces.push(problem.capacity - weight);
            indices[item_idx] = bin_spaces.len() - 1;
        }
    }
    Packing { indices }
}

fn lower_bound(problem: &BBProblem) -> usize {
    let total_weight = problem.weights.iter().sum::<usize>() as f64;
    (total_weight / problem.capacity as f64).ceil() as usize
}

fn local_search(packing: Packing, problem: &BBProblem) -> Packing {
    packing
}

const TIME_LIMIT: u128 = 10_000; // millis
const STOP_TIME: u128 = TIME_LIMIT - 100;

fn find_solution(problem: &BBProblem) -> Packing {
    let lower = lower_bound(problem);
    let mut packing = first_fit(problem);

    let start_time = Instant::now();
    loop {
        if packing.num_bins() == lower { break; }
        else {
            let curr_time = Instant::now();
            let duration = curr_time.duration_since(start_time);
            if duration.as_millis() < STOP_TIME {
                packing = local_search(packing, problem);
            } else {
                break;
            }
        }

    }
    packing
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let capacity: usize = scan.token();
    let weights: Vec<usize> = (0..n).map(|_| scan.token()).collect();
    let problem = BBProblem { capacity, weights };

    let packing = find_solution(&problem);
    println!("{}", packing.num_bins());
    println!("{}", packing);
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
            let result = Self::rol64(self.state[1] * 5, 7) * 9;
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
            self.state = result + 0x9E3779B97f4A7C15;
            result = (result ^ (result >> 30)) * 0xBF58476D1CE4E5B9;
            result = (result ^ (result >> 27)) * 0x94D049BB133111EB;
            return result ^ (result >> 31);
        }
    }
}