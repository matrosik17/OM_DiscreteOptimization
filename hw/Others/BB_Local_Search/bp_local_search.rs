use io::Scanner;
use rng::Xoshiro256ss;
use std::fmt;
use std::fmt::Display;
use std::time::Instant;

struct BPProblem {
    pub capacity: usize,
    pub weights: Vec<usize>,
}

#[derive(Clone)]
struct Packing {
    pub indices: Vec<usize>,
    pub bin_weights: Vec<usize>,
}

impl Packing {
    pub fn from_indices(indices: Vec<usize>, problem: &BPProblem) -> Self {
        let n_bins = *indices.iter().max().unwrap_or(&0) + 1;
        let mut bin_weights = vec![0; n_bins];
        for (item_idx, bin_idx) in indices.iter().enumerate() {
            bin_weights[*bin_idx] += problem.weights[item_idx];
        }
        Self { indices, bin_weights }
    }

    pub fn num_bins(&self) -> usize {
        self.bin_weights.len()
    }
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

fn first_fit(problem: &BPProblem) -> Packing {
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
    Packing::from_indices(indices, problem)
}

fn lower_bound(problem: &BPProblem) -> usize {
    let total_weight = problem.weights.iter().sum::<usize>() as f64;
    (total_weight / problem.capacity as f64).ceil() as usize
}

fn local_search(packing: Packing, problem: &BPProblem, rng: &mut Xoshiro256ss) -> Packing {
    packing
}

const TIME_LIMIT: u128 = 10_000; // millis
const STOP_TIME: u128 = TIME_LIMIT - 100;

fn find_solution(problem: &BPProblem) -> Packing {
    let lower = lower_bound(problem);
    let mut packing = first_fit(problem);

    let start_time = Instant::now();
    let seed: u64 = 42;
    let mut rng = Xoshiro256ss::new(seed);
    loop {
        if packing.num_bins() == lower { break; }
        else {
            let duration = start_time.elapsed();
            if duration.as_millis() < STOP_TIME {
                packing = local_search(packing, problem, &mut rng);
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
    let problem = BPProblem { capacity, weights };

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
            self.state = result.wrapping_add(0x9E3779B97f4A7C15);
            result = (result ^ (result >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            result = (result ^ (result >> 27)).wrapping_mul(0x94D049BB133111EB);
            return result ^ (result >> 31);
        }
    }
}