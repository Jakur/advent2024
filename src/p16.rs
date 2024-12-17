use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Goal,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}

impl Direction {
    fn get_square(self, origin: (usize, usize)) -> (usize, usize) {
        let (row, col) = origin;
        match self {
            Direction::North => (row - 1, col),
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
        }
    }
    fn turn(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Debug)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Position {
    fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }
    fn forward(&self) -> Self {
        let (out_row, out_col) = self.direction.get_square((self.row, self.col));
        Self::new(out_row, out_col, self.direction)
    }
    fn rotate(&self) -> Self {
        Self::new(self.row, self.col, self.direction.turn())
    }
    fn counter_rotate(&self) -> Self {
        Self::new(self.row, self.col, self.direction.turn().turn().turn())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
}

// From https://doc.rust-lang.org/nightly/std/collections/binary_heap/index.html#examples

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: Position,
    cost: usize,
}

impl Edge {
    fn new(node: Position, cost: usize) -> Self {
        Self { node, cost }
    }
}

/// Fixed size history buffer
#[derive(Clone, Debug)]
struct History {
    data: [Position; 4],
    len: usize,
}

impl History {
    fn new() -> Self {
        History {
            data: [Position::new(0, 0, Direction::North); 4],
            len: 0,
        }
    }
    fn push(&mut self, p: Position) {
        self.data[self.len] = p;
        self.len += 1;
    }
    fn clear(&mut self) {
        self.len = 0;
    }
    fn access(&self) -> &[Position] {
        &self.data[0..self.len]
    }
}

struct Grid {
    data: Vec<Tile>,
    num_cols: usize,
    start_row: usize,
    start_col: usize,
}
impl Grid {
    fn new(data: Vec<Tile>, num_rows: usize, start_row: usize, start_col: usize) -> Self {
        let num_cols = data.len() / num_rows;
        Self {
            data,
            num_cols,
            start_row,
            start_col,
        }
    }
    /// Recursively walk all found paths back to the start
    fn visit(&self, idx: usize, history: &[History], visited: &mut HashSet<(usize, usize)>) {
        for p in history[idx].access().iter() {
            visited.insert((p.row, p.col));
            let next = self.get_idx(*p);
            self.visit(next, history, visited);
        }
    }
    fn shortest_path(&self) -> Option<(u64, u64)> {
        let mut prev = vec![History::new(); 4 * self.data.len()];
        let mut dist = vec![usize::MAX; 4 * self.data.len()];

        let start_node = Position::new(self.start_row, self.start_col, Direction::East);
        let start = self.get_idx(start_node);
        let mut heap = BinaryHeap::new();
        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start_node,
        });
        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            if let Tile::Goal = self.get(position.row, position.col) {
                // Compute Part 2
                let end_idx = self.get_idx(position);
                let mut visited = HashSet::new();
                visited.insert((position.row, position.col));
                self.visit(end_idx, &prev, &mut visited);
                return Some((cost as u64, visited.len() as u64));
            }

            // Bad node, look no further
            if cost > dist[self.get_idx(position)] {
                continue;
            }
            let adj_list = [
                Edge::new(position.forward(), 1),
                Edge::new(position.rotate(), 1000),
                Edge::new(position.counter_rotate(), 1000),
            ];
            for edge in &adj_list {
                if let Tile::Wall = self.get(edge.node.row, edge.node.col) {
                    continue;
                }
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                let next_idx = self.get_idx(next.position);
                if next.cost < dist[next_idx] {
                    // Update
                    heap.push(next);
                    prev[next_idx].clear();
                    prev[next_idx].push(position);
                    dist[next_idx] = next.cost;
                } else if next.cost == dist[next_idx] {
                    // Do not need to add the node, but we need to note it for walking backward
                    prev[next_idx].push(position);
                }
            }
        }
        None
    }
    fn get_simple_idx(&self, row: usize, col: usize) -> usize {
        row * self.num_cols + col
    }
    fn get_idx(&self, p: Position) -> usize {
        p.row * (4 * self.num_cols) + p.col * 4 + p.direction as usize
    }
    fn get(&self, row: usize, col: usize) -> Tile {
        self.data[self.get_simple_idx(row, col)]
    }
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut data = Vec::new();
    let mut num_rows = 0;
    let mut start_row = 0;
    let mut start_col = 0;
    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'E' => Tile::Goal,
                'S' => {
                    start_row = num_rows;
                    start_col = idx;
                    Tile::Empty
                }
                _ => unimplemented!(),
            };
            data.push(tile);
        }
        num_rows += 1;
    }
    let grid = Grid::new(data, num_rows, start_row, start_col);
    let (part1, part2) = grid.shortest_path()?;
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p16.txt");
    #[test]
    fn day16_solve() {
        dbg!(super::solve(INPUT));
    }
}
