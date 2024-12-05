use itertools::Itertools;

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_whitespace().collect_tuple()?;
        left.push(a.parse().ok()?);
        right.push(b.parse().ok()?)
    }
    left.sort();
    right.sort();
    let part1: i32 = left
        .iter()
        .copied()
        .zip(right.iter().copied())
        .map(|(a, b)| (a - b).abs())
        .sum();
    /*
    This is more complicated than it needs to be, but with a sorted list
    we can binary search efficiently without needing to create a secondary
    data structure.
    */

    let counts = left.iter().map(|x| {
        right
            .binary_search(x)
            .map(|idx| {
                let real_st = walk(idx, &right);
                count(real_st, &right)
            })
            .unwrap_or(0)
    });
    let part2 = left
        .iter()
        .copied()
        .zip(counts)
        .map(|(a, b)| a * b as i32)
        .sum();
    Some((part1, part2))
}

fn count(st: usize, slice: &[i32]) -> usize {
    slice[st..]
        .iter()
        .take_while(|&&val| val == slice[st])
        .count()
}

/// Walk backwards to the first (leftmost) matching instance in the list.
fn walk(st: usize, slice: &[i32]) -> usize {
    (0..=st)
        .rev()
        .take_while(|&idx| slice[idx] == slice[st])
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p1.txt");
    #[test]
    fn day1_solve() {
        dbg!(super::solve(INPUT));
    }
}
