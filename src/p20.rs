use crate::Direction;
use std::collections::VecDeque;

const MAX_CHEAT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
struct SearchNode {
    node: (usize, usize),
    dist: usize,
}

impl SearchNode {
    fn new(node: (usize, usize), dist: usize) -> Self {
        Self { node, dist }
    }
}

struct Origin((usize, usize));

impl IntoIterator for Origin {
    type Item = (usize, usize);

    type IntoIter = DistanceIterator;

    fn into_iter(self) -> Self::IntoIter {
        DistanceIterator::new(self.0)
    }
}

struct DistanceIterator {
    max_row: usize,
    max_col: usize,
    row: usize,
    col: usize,
}

impl DistanceIterator {
    fn new(origin: (usize, usize)) -> Self {
        let (origin_row, origin_col) = origin;
        let min_row = origin_row.saturating_sub(MAX_CHEAT + 1);
        let min_col = origin_col.saturating_sub(MAX_CHEAT + 1);
        let max_row = origin_row + MAX_CHEAT + 1;
        let max_col = origin_col + MAX_CHEAT + 1;
        Self {
            max_row,
            max_col,
            row: min_row,
            col: min_col,
        }
    }
}

impl std::iter::Iterator for DistanceIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.max_row {
            return None;
        }
        let out = (self.row, self.col);
        if self.col >= self.max_col {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(out)
    }
}

struct Grid {
    data: Vec<Tile>,
    num_cols: usize,
    num_rows: usize,
    end: (usize, usize),
}
impl Grid {
    fn new(data: Vec<Tile>, num_rows: usize, end: (usize, usize)) -> Self {
        let num_cols = data.len() / num_rows;
        Self {
            data,
            num_cols,
            num_rows,
            end,
        }
    }
    fn all_shortest(&self) -> Vec<u64> {
        let mut out = vec![u64::MAX; self.data.len()];
        self.bfs(&mut out); // BFS gives optimal shortest distances in an unweighted graph
        out
    }
    fn bfs(&self, distances: &mut [u64]) {
        // Memory should not explode because the grid isn't that big
        let mut queue = VecDeque::new();
        distances[self.get_simple_idx(self.end.0, self.end.1)] = 0;
        queue.push_back(SearchNode::new(self.end, 0));
        while let Some(SearchNode { node, dist }) = queue.pop_front() {
            let neighbors = Direction::orthogonal(node);
            for next in neighbors {
                let (n_row, n_col) = next;
                if let Tile::Empty = self.get(n_row, n_col) {
                    let neighbor_idx = self.get_simple_idx(n_row, n_col);
                    // If the distance is max, we have not visited it yet
                    if distances[neighbor_idx] == u64::MAX {
                        queue.push_back(SearchNode::new(next, dist + 1));
                        distances[self.get_simple_idx(n_row, n_col)] = (dist + 1) as u64;
                    }
                }
            }
        }
    }
    fn solve_part1(&self, distances: &[u64]) -> u64 {
        self.find_cheats(distances, 2, Direction::wide_orthongonal)
    }
    fn solve_part2(&self, distances: &[u64]) -> u64 {
        self.find_cheats(distances, MAX_CHEAT, |x| Origin(x))
    }
    fn find_cheats<I, F>(&self, distances: &[u64], cheat_dist: usize, construct: F) -> u64
    where
        I: IntoIterator<Item = (usize, usize)>,
        F: Fn((usize, usize)) -> I,
    {
        let mut count = 0;
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                if let Tile::Empty = self.get(row, col) {
                    let dist1 = distances[self.get_simple_idx(row, col)];
                    let reachable = construct((row, col));
                    for (n_row, n_col) in reachable {
                        // Check in bounds, and that it is empty
                        if self.is_empty(n_row, n_col) {
                            let dist2 = distances[self.get_simple_idx(n_row, n_col)];
                            let manhattan_dist = n_row.abs_diff(row) + n_col.abs_diff(col);
                            if manhattan_dist > cheat_dist {
                                continue;
                            }
                            if dist2
                                .saturating_sub(dist1)
                                .saturating_sub(manhattan_dist as u64)
                                >= 100
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }
    fn is_empty(&self, row: usize, col: usize) -> bool {
        row < self.num_rows && col < self.num_cols && self.get(row, col) != Tile::Wall
    }
    fn get_simple_idx(&self, row: usize, col: usize) -> usize {
        row * self.num_cols + col
    }
    fn get(&self, row: usize, col: usize) -> Tile {
        self.data[self.get_simple_idx(row, col)]
    }
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut end = (0, 0);
    let mut grid_data = Vec::new();
    let mut num_rows = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.as_bytes().iter().copied().enumerate() {
            let tile = match c {
                b'#' => Tile::Wall,
                b'.' => Tile::Empty,
                b'S' => Tile::Empty,
                b'E' => {
                    end = (row, col);
                    Tile::Empty
                }
                _ => unimplemented!(),
            };
            grid_data.push(tile);
        }
        num_rows += 1;
    }
    let grid = Grid::new(grid_data, num_rows, end);
    let all_shortest = grid.all_shortest();
    let part1 = grid.solve_part1(&all_shortest);
    let part2 = grid.solve_part2(&all_shortest);
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p20.txt");
    #[test]
    fn day20_solve() {
        dbg!(super::solve(INPUT));
    }
}
