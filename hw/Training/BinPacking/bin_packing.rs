use io::Scanner;
use std::fmt;
use std::fmt::Display;
// use std::io::Write;

struct BBProblem {
    pub capacity: usize,
    pub weights: Vec<usize>,
}

#[derive(Clone)]
struct Packing {
    pub indices: Vec<usize>,
}

impl Packing {
    pub fn with_capacity(size: usize) -> Self {
        Self { indices: Vec::with_capacity(size) }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn put_item(&mut self, bin_idx: usize) {
        self.indices.push(bin_idx);
    }

    pub fn truncate(&mut self, len: usize) {
        self.indices.truncate(len);
    }

    pub fn num_bins(&self) -> usize {
        *self.indices.iter().max().unwrap_or(&0) + 1
    }

    pub fn is_correct_packing(&self, problem: &BBProblem) -> bool {
        let mut bins = vec![0_usize; self.num_bins()];
        for (item_idx, bin_idx) in self.indices.iter().enumerate() {
            let bin_weight = &mut bins[*bin_idx];
            *bin_weight += problem.weights[item_idx];
            if *bin_weight > problem.capacity { return false; }
        }
        true
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

fn bin_packing_impl(
    lower_bound: usize,
    curr_packing: &mut Packing,
    best_packing: &mut Packing,
    problem: &BBProblem
) {
    if best_packing.num_bins() == lower_bound { return; }
    else if !curr_packing.is_correct_packing(problem) { return; }
    else if curr_packing.num_bins() >= best_packing.num_bins() { return;}
    else if curr_packing.len() == problem.weights.len() { *best_packing = curr_packing.clone(); }
    else {
        let packing_len = curr_packing.len();
        for bin_idx in 0..best_packing.num_bins() {
            curr_packing.truncate(packing_len);
            curr_packing.put_item(bin_idx);
            bin_packing_impl(lower_bound, curr_packing, best_packing, problem);
        }
    }
}

fn bin_packing(problem: &BBProblem) -> Packing {
    let lower = lower_bound(problem);
    let mut best_packing = first_fit(problem);
    let mut curr_packing = Packing::with_capacity(best_packing.len());

    bin_packing_impl(lower, &mut curr_packing, &mut best_packing, problem);

    best_packing
}

fn print_bins(packing: &Packing, problem: &BBProblem) {
    let num_bins = packing.num_bins();
    let mut bins = vec![Vec::<usize>::new(); num_bins];

    for (item_idx, bin_idx) in packing.indices.iter().enumerate() {
        bins[*bin_idx].push(problem.weights[item_idx]);
    }

    for bin in bins {
        let result_str = bin.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        // let _ = writeln!("{}", result_str);
        println!("{}", result_str);
    }

}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    // let stdout = std::io::stdout();
    // let mut writer = std::io::BufWriter::new(stdout.lock());

    let n: usize = scan.token();
    let capacity: usize = scan.token();
    let weights: Vec<usize> = (0..n).map(|_| scan.token()).collect();
    let problem = BBProblem { capacity, weights };

    let packing = bin_packing(&problem);
    // println!("{}", packing);
    print_bins(&packing, &problem);
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