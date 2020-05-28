use io::Scanner;
use rng::Xoshiro256ss;
use std::fmt;
use std::fmt::Display;
use std::time::{Instant, Duration};

struct BPProblem {
    pub capacity: usize,
    pub weights: Vec<usize>,
}

#[derive(Default, Clone)]
struct Bin {
    pub items: Vec<usize>,
}

impl Bin {
    pub fn weight(&self, problem: &BPProblem) -> usize {
        self.items.iter().map(|item_idx| problem.weights[*item_idx]).sum()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Clone)]
struct Packing {
    pub bins: Vec<Bin>,
}

impl Packing {
    pub fn from_indices(indices: Vec<usize>) -> Self {
        let n_bins = *indices.iter().max().unwrap_or(&0) + 1;
        let mut bins = vec![Bin::default(); n_bins];
        for (item_idx, bin_idx) in indices.iter().enumerate() {
            bins[*bin_idx].items.push(item_idx);
        }
        Self { bins }
    }

    pub fn num_bins(&self) -> usize {
        self.bins.len()
    }

    pub fn len(&self) -> usize {
        self.bins.len()
    }
}

impl Display for Packing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bin_dist: Vec<(usize, usize)> = self.bins.iter()
            .enumerate()
            .flat_map(|(bin_idx, bin)| {
                bin.items.iter()
                .map(move |item_idx| (*item_idx, bin_idx))
            })
            .collect();

        bin_dist.sort_unstable_by(|(item1, _), (item2, _)| {
            item1.cmp(item2)
        });

        let result_str = bin_dist.into_iter()
            .map(|(_, bin_idx)| (bin_idx + 1).to_string())
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
    Packing::from_indices(indices)
}

fn lower_bound(problem: &BPProblem) -> usize {
    let total_weight = problem.weights.iter().sum::<usize>() as f64;
    (total_weight / problem.capacity as f64).ceil() as usize
}

enum SearchStrategy {
    Rebalance(usize), // пробуем перераспределить обьекты между контейнерами
    Disbalance(usize, usize), // пробуем разгрузить один контейнер и загрузить другой
}

fn rebalance_bins(
    bin_idx: usize,
    mut packing: Packing,
    problem: &BPProblem,
    rng: &mut Xoshiro256ss
) -> Packing {
    let bin_weight = packing.bins[bin_idx].weight(problem);

    let other_bins_indices: Vec<usize> = (0..packing.len())
        .filter(|x| *x != bin_idx)
        .collect();

    if bin_weight > problem.capacity / 2 { // пытаемся дозаполнить контейнер
        let mut free_space = problem.capacity - bin_weight;
        for &other_bin_idx in other_bins_indices.iter() {
            let bin_item_idx = rng.rand() as usize % packing.bins[other_bin_idx].len();
            let item_idx = packing.bins[other_bin_idx].items[bin_item_idx];
            let item_weight = problem.weights[item_idx];

            if item_weight < free_space {
                free_space -= item_weight;
                packing.bins[other_bin_idx].items.remove(bin_item_idx);
                packing.bins[bin_idx].items.push(item_idx);
            }
        }
    } else { // пытаемся разгрузить контейнер
        for &other_bin_idx in other_bins_indices.iter() {
            let other_bin_weight = packing.bins[other_bin_idx].weight(problem);
            let mut other_free_space = problem.capacity - other_bin_weight;

            // выбираем обьекты для переноса в другой контейнер
            let move_bin_items_indices: Vec<usize> = packing.bins[bin_idx].items.iter()
                .enumerate()
                .filter_map(|(bin_item_idx, &item_idx)| {
                    let item_weight = problem.weights[item_idx];
                    if item_weight < other_free_space {
                        other_free_space -= item_weight;
                        Some(bin_item_idx)
                    } else {
                        None
                    }
                })
                .collect();

            // переносим обьекты
            for bin_item_idx in move_bin_items_indices {
                let item_idx = packing.bins[bin_idx].items.remove(bin_item_idx);
                packing.bins[other_bin_idx].items.push(item_idx);
            }
        }
        if packing.bins[bin_idx].is_empty() { packing.bins.remove(bin_idx); }
    }
    packing
}

fn disbalance_bins(
    bins_idx: (usize, usize),
    mut packing: Packing,
    problem: &BPProblem,
    rng: &mut Xoshiro256ss
) -> Packing {
    let (bin1_idx, bin2_idx) = bins_idx;
    let weight1 = packing.bins[bin1_idx].weight(problem);
    let weight2 = packing.bins[bin2_idx].weight(problem);

    let (bin1_idx, bin2_idx) = if weight1 >= weight2 {
        (bin1_idx, bin2_idx)
    } else {
        (bin2_idx, bin1_idx)
    };

    // вычисляем свободное место в большем контейнере
    let free_space1 = problem.capacity - packing.bins[bin1_idx].weight(problem);
    if free_space1 == 0 { return packing; }
    // выбираем случайный обьект из меньшего контейнера
    let item_from2_idx = rng.rand() as usize % packing.bins[bin2_idx].len();
    let item_from2 = packing.bins[bin2_idx].items[item_from2_idx];
    // пробуем переложить обьект
    if problem.weights[item_from2] < free_space1 {
        packing.bins[bin1_idx].items.push(item_from2);
        packing.bins[bin2_idx].items.remove(item_from2_idx);
        if packing.bins[bin2_idx].is_empty() { packing.bins.remove(bin2_idx); }
    } else { // если не удается, то пытаемся обменяться обьектами с увеличением веса первого контейнера
        let min_weight = problem.weights[item_from2] - free_space1;
        let max_weight = problem.weights[item_from2];
        let swap_element = packing.bins[bin1_idx].items.iter()
            .enumerate()
            .find_map(|(idx, &item_idx)| {
                let weight = problem.weights[item_idx];
                if min_weight <= weight && weight < max_weight {
                    Some((idx, item_idx))
                } else {
                    None
                }
            });

        if let Some((item_from1_idx, item_from1)) = swap_element {
            packing.bins[bin1_idx].items.remove(item_from1_idx);
            packing.bins[bin2_idx].items.remove(item_from2_idx);

            packing.bins[bin1_idx].items.push(item_from2);
            packing.bins[bin2_idx].items.push(item_from1);
        }
    }
    packing
}

fn local_search(packing: Packing, problem: &BPProblem, rng: &mut Xoshiro256ss) -> Packing {
    let n_bins = packing.num_bins();
    let bin_idx_pair = (rng.rand() as usize % n_bins, rng.rand() as usize % n_bins);

    let strategy = match bin_idx_pair {
        (n, m) if n == m => SearchStrategy::Rebalance(n),
        (n, m) => SearchStrategy::Disbalance(n, m),
    };

    match strategy {
        SearchStrategy::Rebalance(n) => rebalance_bins(n, packing, problem, rng),
        SearchStrategy::Disbalance(n, m) => disbalance_bins((n, m), packing, problem, rng),
    }
}

const TIME_LIMIT: u64 = 10_000;
const STOP_TIME: Duration = Duration::from_millis(TIME_LIMIT - 1);

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
            if duration < STOP_TIME {
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
    // println!("{}", packing.num_bins());
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
