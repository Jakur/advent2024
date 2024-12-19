const NUM_COLORS: usize = 5;

fn get_color_idx(b: u8) -> usize {
    match b {
        b'w' => 0,
        b'u' => 1,
        b'b' => 2,
        b'r' => 3,
        b'g' => 4,
        _ => unimplemented!(),
    }
}

fn parse_string(s: &[u8], valid: &[Vec<&[u8]>; NUM_COLORS]) -> u64 {
    let mut solves = vec![0; s.len() + 1];
    solves[s.len()] = 1;
    for i in (0..s.len()).rev() {
        let first = get_color_idx(s[i]);
        for &legal in valid[first].iter() {
            let end_idx = i + legal.len();
            if end_idx > s.len() {
                break; // Trying to match a string that is longer than our substring
            }
            if solves[end_idx] == 0 {
                continue; // Even if we match this substring, the end would not be valid
            }
            let substr = &s[i..end_idx];
            if substr == legal {
                // Found a valid match
                solves[i] += solves[end_idx];
            }
        }
    }
    solves[0]
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut strings: [Vec<&[u8]>; NUM_COLORS] = Default::default();
    for data in input.lines().nth(0)?.split(", ") {
        let bytes = data.as_bytes();
        let first = bytes[0];
        strings[get_color_idx(first)].push(bytes);
    }
    for v in strings.iter_mut() {
        v.sort_by_key(|s| s.len());
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for s in input.lines().skip(2) {
        let bytes = s.as_bytes();
        let outcome = parse_string(bytes, &strings);
        if outcome > 0 {
            part1 += 1;
        }
        part2 += outcome;
    }
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p19.txt");
    #[test]
    fn day19_solve() {
        dbg!(super::solve(INPUT));
    }
}
