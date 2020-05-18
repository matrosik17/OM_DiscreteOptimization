use std::collections::HashMap;

type Item = (usize, usize);
type UpperBoundMem = HashMap<(usize, usize), usize>;

fn lower_bound(mut knapsack_volume: usize, sorted_items: &[Item]) -> usize {
    let mut max_value: usize = 0;

    for (volume, value) in sorted_items.iter() {
        if *volume <= knapsack_volume {
            knapsack_volume -= *volume;
            max_value += *value;
        }
    }
    max_value
}

fn upper_bound_mem(knapsack_volume: usize, sorted_items: &[Item], mem: &mut UpperBoundMem) -> usize {
    if let Some(upper_bound) = mem.get(&(knapsack_volume, sorted_items.len())) { return *upper_bound; }

    let mut curr_volume = knapsack_volume;
    let mut max_value = 0;

    for (volume, value) in sorted_items.iter() {
        if *volume < curr_volume {
            curr_volume -= volume;
            max_value += value;
        } else {
            let item_density = *value as f64 / *volume as f64;
            max_value += (item_density * curr_volume as f64).ceil() as usize;
            break;
        }
    }
    mem.insert((knapsack_volume, sorted_items.len()), max_value);
    max_value
}

fn knapsack_impl(
    current_value: usize,
    knapsack_volume: usize,
    items: &[Item],
    lower_bound: &mut usize,
    mem: &mut UpperBoundMem
) -> usize {
    if items.len() == 0 { 0 }
    else {
        let ubound_estimation: usize = upper_bound_mem(knapsack_volume, &items, mem);
        if current_value + ubound_estimation <= *lower_bound { 0 }
        else {
            let (volume, value) = items[0];
            let result = if volume <= knapsack_volume {
                std::cmp::max(
                    value + knapsack_impl(current_value + value, knapsack_volume - volume, &items[1..], lower_bound, mem),
                    knapsack_impl(current_value, knapsack_volume, &items[1..], lower_bound, mem)
                )
            } else {
                knapsack_impl(current_value, knapsack_volume, &items[1..], lower_bound, mem)
            };

            if result > *lower_bound { *lower_bound = result; }
            result
        }
    }
}

fn knapsack(knapsack_volume: usize, items: &[Item]) -> usize {
    let mut max_value = lower_bound(knapsack_volume, &items);
    let mut mem = UpperBoundMem::with_capacity(2 * items.len());
    knapsack_impl(0, knapsack_volume, &items, &mut max_value, &mut mem);
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
        items.push(item);
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