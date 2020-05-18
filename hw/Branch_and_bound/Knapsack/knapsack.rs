use std::collections::HashMap;

type Item = (usize, usize);
type UpperBoundMem = HashMap<(usize, usize), usize>;

fn calc_bounds(knapsack_volume: usize, sorted_items: &[Item]) -> (usize, usize) {
    let mut curr_volume = knapsack_volume;
    let mut lower_bound: usize = 0;
    let mut upper_bound = None;

    for (volume, value) in sorted_items.iter() {
        if *volume <= curr_volume {
            curr_volume -= *volume;
            lower_bound += *value;
        } else {
            if lower_bound != 0 && upper_bound == None {
                let item_density = *value as f64 / *volume as f64;
                let last_part_value = (item_density * curr_volume as f64).ceil() as usize;
                upper_bound = Some(lower_bound + last_part_value);
            }
        }
    }
    (lower_bound, upper_bound.unwrap_or(lower_bound))
}

fn knapsack_impl(
    current_value: usize,
    knapsack_volume: usize,
    items: &[Item],
    max_value: &mut usize
) -> usize {
    if items.len() == 0 { return 0; }

    let (lower_bound, upper_bound) = calc_bounds(knapsack_volume, &items);
    *max_value = std::cmp::max(*max_value, current_value + lower_bound);
    if current_value + upper_bound <= *max_value { return 0; }

    let min_volume_addition = items.iter().map(|x| x.0).min().unwrap();
    if knapsack_volume < min_volume_addition { return 0; }

    let (volume, value) = items[0];
    let result = if volume <= knapsack_volume {
        std::cmp::max(
            value + knapsack_impl(current_value + value, knapsack_volume - volume, &items[1..], max_value),
            knapsack_impl(current_value, knapsack_volume, &items[1..], max_value)
        )
    } else {
        knapsack_impl(current_value, knapsack_volume, &items[1..], max_value)
    };

    if result > *max_value { *max_value = result; }
    result
}

fn knapsack(knapsack_volume: usize, items: &[Item]) -> usize {
    let (mut max_value, _) = calc_bounds(knapsack_volume, &items);
    knapsack_impl(0, knapsack_volume, &items, &mut max_value);
    max_value
}

fn main() {
    let stdin = std::io::stdin();
    let mut scan = io::Scanner::new(stdin.lock());

    let knapsack_volume: usize = scan.token();
    let num_items: usize = scan.token();

    let mut items: Vec<Item> = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        let item: Item = (scan.token(), scan.token());
        if item.0 <= knapsack_volume {
            items.push(item);
        }
    }

    items.sort_unstable_by(|(w1, v1), (w2, v2)| {
        let density1 = *v1 as f64 / *w1 as f64;
        let density2 = *v2 as f64 / *w2 as f64;
        density2.partial_cmp(&density1).unwrap()
    });

    println!("{}", knapsack(knapsack_volume, &items));
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