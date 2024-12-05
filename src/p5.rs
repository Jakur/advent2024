use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Edge {
    src: usize,
    dest: usize,
}

impl Edge {
    fn is_obeyed(&self, data: &[usize]) -> bool {
        data.iter().position(|&x| x == self.src).unwrap()
            < data.iter().position(|&x| x == self.dest).unwrap()
    }
}

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut max_edge = 0;
    let rules: Vec<_> = input
        .lines()
        .filter_map(|x| {
            let data = x.split("|").map(|x| x.parse().ok()).collect_tuple();
            if let Some((Some(src), Some(dest))) = data {
                max_edge = std::cmp::max(max_edge, src);
                max_edge = std::cmp::max(max_edge, dest);
                Some(Edge { src, dest })
            } else {
                None
            }
        })
        .collect();
    // Adjacency List
    let mut edge_rules = vec![Vec::new(); max_edge + 1];
    for rule in rules {
        edge_rules[rule.src].push(rule);
    }
    let lists: Vec<Vec<_>> = input
        .lines()
        .filter_map(|x| {
            if x.contains(",") {
                let vec = x.split(",").map(|x| x.parse::<usize>().ok()).collect();
                vec
            } else {
                None
            }
        })
        .collect();
    for list in lists.iter() {
        // Filter out only rules where both src and dest appear in our list
        let mut relevant_rules = Vec::new();
        for val in list {
            for rule in edge_rules[*val as usize].iter() {
                if list.contains(&rule.dest) {
                    relevant_rules.push(rule);
                }
            }
        }
        if relevant_rules.iter().all(|x| x.is_obeyed(&list)) {
            part1 += list[list.len() / 2];
        } else {
            part2 += topsort_middle(&mut relevant_rules, &list);
        }
    }
    Some((part1, part2))
}

fn topsort_middle(relevant_rules: &mut Vec<&Edge>, list: &[usize]) -> usize {
    // Kahn's Algorithm, cutoff halfway through
    let is_root = |rules: &[&Edge], x| rules.iter().find(|y| y.dest == x).is_none();
    let mut out = Vec::new();
    let mut start = list
        .iter()
        .filter(|&&x| is_root(relevant_rules, x))
        .collect_vec();
    assert!(start.len() * 2 < list.len()); // Middle would be non-deterministic
    let target = list.len() / 2;
    while let Some(node) = start.pop() {
        out.push(*node);
        if out.len() > target {
            break; // We can get the correct middle point now, which is all we need
        }
        while let Some(edge_idx) = relevant_rules.iter().position(|x| x.src == *node) {
            let edge = relevant_rules[edge_idx];
            relevant_rules.swap_remove(edge_idx);
            if is_root(relevant_rules, edge.dest) {
                start.push(&edge.dest);
            }
        }
    }
    *out.last().unwrap()
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p5.txt");
    #[test]
    fn day5_solve() {
        dbg!(super::solve(INPUT));
    }
}
