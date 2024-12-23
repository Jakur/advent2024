use std::collections::HashMap;

fn mix(secret: u64, other: u64) -> u64 {
    secret ^ other
}

fn prune(secret: u64) -> u64 {
    secret % 16777216 // 2^24
}

fn generate(x: u64) -> u64 {
    let x2 = prune(mix(x, x * 64));
    let x3 = prune(mix(x2, x2 / 32));
    let x4 = prune(mix(x3, x3 * 2048));
    return x4;
}

pub fn solve(input: &str) -> Option<(u64, i32)> {
    let mut part1 = 0;
    let mut global: HashMap<_, i32> = HashMap::new();
    for line in input.lines() {
        let num = line.parse().ok()?;
        let nums: Vec<_> = (0..2000)
            .scan(num, |acc, _| {
                let outcome = generate(*acc);
                *acc = outcome;
                Some(outcome)
            })
            .collect();

        let mut local = HashMap::new();
        for window in nums.windows(5) {
            let mut diffs = [0; 4];
            let mut idx = 0;
            let mut value = 0;
            // This is somewhat inefficient because it recomputes the diffs unnecessarily
            for (prev, next) in window.iter().copied().zip(window.iter().copied().skip(1)) {
                let prev = (prev % 10) as i32;
                let next = (next % 10) as i32;
                let diff = next - prev;
                diffs[idx] = diff;
                idx += 1;
                value = next;
            }
            assert_eq!(idx, 4);
            local.entry(diffs).or_insert(value);
        }
        // We just need to sum the contributions of all the local hashmaps
        for (k, v) in local {
            *global.entry(k).or_default() += v;
        }
        part1 += nums.last()?;
    }
    let part2 = *global.values().max()?;
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p22.txt");
    #[test]
    fn day22_solve() {
        // crate::simple_bench(INPUT, super::solve);
        dbg!(super::solve(INPUT));
    }
}
