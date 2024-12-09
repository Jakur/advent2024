use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point2D {
    row: usize,
    col: usize,
}

impl Point2D {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl std::ops::Add<Point2D> for Point2D {
    type Output = Self;

    fn add(self, rhs: Point2D) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl std::ops::Sub<Point2D> for Point2D {
    type Output = Self;

    fn sub(self, rhs: Point2D) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let mut counter = 0;
    let mut num_rows = 0;
    let mut map: HashMap<char, Vec<Point2D>> = HashMap::new();
    let mut targets = HashSet::new();
    let mut targets2 = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            counter += 1;
            if c != '.' {
                map.entry(c).or_default().push(Point2D::new(row, col));
            }
        }
        num_rows += 1;
    }
    assert_eq!(num_rows * num_rows, counter); // Is square
    let in_bounds = |dest: Point2D| dest.row < num_rows && dest.col < num_rows;
    for (_k, vec) in map.iter().filter(|(_, vec)| vec.len() > 1) {
        for (i, val1) in vec.iter().copied().enumerate() {
            targets2.insert(val1);
            for (j, val2) in vec.iter().copied().enumerate() {
                if i == j {
                    continue;
                }
                let dist = val2 - val1;
                let mut dest1 = val2 + dist;
                let mut dest2 = val1 - dist;
                // Direction 1
                if in_bounds(dest1) {
                    targets.insert(dest1);
                    loop {
                        targets2.insert(dest1);
                        dest1 = dest1 + dist;
                        if !in_bounds(dest1) {
                            break;
                        }
                    }
                }
                // Direction 2
                if in_bounds(dest2) {
                    targets.insert(dest2);
                    loop {
                        targets2.insert(dest2);
                        dest2 = dest2 - dist;
                        if !in_bounds(dest2) {
                            break;
                        }
                    }
                }
            }
        }
    }
    Some((targets.len(), targets2.len()))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p8.txt");
    #[test]
    fn day8_solve() {
        dbg!(super::solve(INPUT));
    }
}
