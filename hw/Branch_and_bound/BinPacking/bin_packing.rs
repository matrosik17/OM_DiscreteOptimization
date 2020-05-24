use io::Scanner;

fn lower_bound(capacity: usize, weights: &[usize]) -> usize {
    let total_weight = weights.iter().sum::<usize>() as f64;
    (total_weight / capacity as f64).ceil() as usize
}

fn first_fit(capacity: usize, weights: &[usize]) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![std::usize::MAX; weights.len()];
    let mut bin_spaces: Vec<usize> = Vec::with_capacity(indices.len());

    for (item_idx, weight) in weights.iter().enumerate() {
        let bin = bin_spaces.iter_mut()
            .enumerate()
            .find_map(|(bin_idx, x)| {
                if *x > *weight {
                    *x -= *weight;
                    indices[item_idx] = bin_idx + 1;
                    Some(x)
                } else {
                    None
                }
            });

        if bin == None {
            bin_spaces.push(capacity - weight);
            indices[item_idx] = bin_spaces.len();
        }
    }
    indices
}

fn upper_bound(capacity: usize, weights: &[usize]) -> usize {
    let indices = first_fit(capacity, weights);
    *indices.iter().max().unwrap_or(&0)
}

// fn bin_packing(capacity: usize, weights: &[usize]) -> Vec<usize> {

// }

fn main() {
    let stdin = std::io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    let capacity: usize = scan.token();

    let mut weights: Vec<usize> = (0..n).map(|_| scan.token()).collect();
    weights.sort_unstable();

    println!("{:?}", weights);
    println!("{:?}", lower_bound(capacity, &weights));
    println!("{:?}", first_fit(capacity, &weights));
    println!("{:?}", upper_bound(capacity, &weights));
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