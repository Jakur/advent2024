pub fn solve(input: &str) -> Option<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let vec = parse_line(line);
            let (part1, part2) = is_gradual(&vec);
            (part1 as usize, part2 as usize)
        })
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
}

fn parse_line(line: &str) -> Vec<i32> {
    let diffs: Vec<_> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .scan(0, |state, val| {
            let diff = *state - val;
            *state = val;
            Some(diff)
        })
        .collect();
    diffs
}

fn is_gradual(diffs: &[i32]) -> (bool, bool) {
    let f1 = |&diff| (diff > 0) && (diff <= 3);
    let f2 = |&diff| (diff < 0) && (diff >= -3);
    let incr = diffs.iter().skip(1).all(f1);
    let decr = diffs.iter().skip(1).all(f2);
    let part1 = incr || decr;
    if !part1 {
        // Relax the restrictions
        let mut part2 = false;
        let incr = diffs.iter().skip(2).all(f1);
        let decr = diffs.iter().skip(2).all(f2);
        // Try removing element 0
        if incr || decr {
            return (true, true);
        }
        // If not, try removing the remaining
        for i in 1..diffs.len() {
            let mut changed = Vec::from_iter(diffs.iter().copied());
            let val = changed[i];
            if let Some(x) = changed.get_mut(i + 1) {
                *x += val;
            }
            changed.swap_remove(i);
            let incr = changed
                .iter()
                .skip(1)
                .all(|&diff| (diff > 0) && (diff <= 3)); // Random lifetime issue if using f1
            let decr = changed
                .iter()
                .skip(1)
                .all(|&diff| (diff < 0) && (diff >= -3)); // Random lifetime issue if using f2
            if incr || decr {
                part2 = true;
                break;
            }
        }
        (false, part2)
    } else {
        (true, true)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p2.txt");
    #[test]
    fn day2_solve() {
        dbg!(super::solve(INPUT));
    }
}
