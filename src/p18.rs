use super::Direction;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const NUM_ROWS: usize = 71;
const NUM_COLS: usize = 71;

#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
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

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Goal,
}

struct Grid {
    data: Vec<Tile>,
    num_cols: usize,
    remaining_commands: Vec<(usize, usize)>,
    solve_cache: Vec<Option<bool>>,
    last_ptr: usize,
}
impl Grid {
    fn new(mut commands: Vec<(usize, usize)>) -> Self {
        let mut data = vec![Tile::Empty; NUM_COLS * NUM_ROWS];
        let num_cols = NUM_COLS;
        *data.last_mut().unwrap() = Tile::Goal;
        let mut grid = Self {
            data,
            num_cols,
            last_ptr: 0,
            solve_cache: vec![None; commands.len() - 1024],
            remaining_commands: Vec::new(), // Slight lifetime hack
        };
        for (row, col) in commands.drain(0..1024) {
            grid.set_wall(row, col);
        }
        grid.remaining_commands = commands;
        grid
    }
    fn first_failure(&mut self, idx: usize) -> Ordering {
        let outcome = self.do_solve(idx);
        if outcome {
            // True, means it is still solvable, i.e. Less
            Ordering::Less
        } else {
            // Not solvable, check if its left neighbor was solvable
            if self.do_solve(idx - 1) {
                // We're done
                Ordering::Equal
            } else {
                // Both are unsolvable, continue
                Ordering::Greater
            }
        }
    }
    fn do_solve(&mut self, idx: usize) -> bool {
        if let Some(hit) = self.solve_cache[idx] {
            return hit;
        }
        self.set_state(idx);
        let outcome = self.shortest_path().is_some();
        self.solve_cache[idx] = Some(outcome);
        outcome
    }
    fn set_state(&mut self, idx: usize) {
        if idx > self.last_ptr {
            for i in self.last_ptr..=idx {
                let (row, col) = self.remaining_commands[i];
                self.set_wall(row, col);
            }
        } else {
            for i in idx..=self.last_ptr {
                let (row, col) = self.remaining_commands[i];
                self.unset_wall(row, col);
            }
        }
        self.last_ptr = idx;
    }
    fn unset_wall(&mut self, row: usize, col: usize) {
        *self.get_mut(row, col) = Tile::Empty;
    }
    fn set_wall(&mut self, row: usize, col: usize) {
        *self.get_mut(row, col) = Tile::Wall;
    }
    fn shortest_path(&self) -> Option<u64> {
        // Djikstra's algorithm is not necessary here because the graph is unweighted
        // But I didn't know if Part 2 would change that, and I started with Day 16's
        // Code as a baseline
        let mut dist = vec![usize::MAX; 4 * self.data.len()];

        let start_node = Position::new(0, 0);
        let start = self.get_pos_idx(start_node);
        let mut heap = BinaryHeap::new();
        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start_node,
        });
        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            if let Tile::Goal = self.get(position.row, position.col) {
                return Some(cost as u64);
            }

            // Bad node, look no further
            if cost > dist[self.get_pos_idx(position)] {
                continue;
            }
            let adj_list = Direction::orthogonal((position.row, position.col));
            for edge in &adj_list {
                // Check in bounds
                if edge.0 >= NUM_ROWS || edge.1 >= NUM_COLS {
                    continue;
                }
                if let Tile::Wall = self.get(edge.0, edge.1) {
                    continue;
                }
                let next = State {
                    cost: cost + 1,
                    position: Position::new(edge.0, edge.1),
                };

                let next_idx = self.get_pos_idx(next.position);
                if next.cost < dist[next_idx] {
                    // Update
                    heap.push(next);
                    dist[next_idx] = next.cost;
                }
            }
        }
        None
    }
    fn get_pos_idx(&self, pos: Position) -> usize {
        self.get_simple_idx(pos.row, pos.col)
    }
    fn get_simple_idx(&self, row: usize, col: usize) -> usize {
        row * self.num_cols + col
    }
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Tile {
        let idx = self.get_simple_idx(row, col);
        &mut self.data[idx]
    }
    fn get(&self, row: usize, col: usize) -> Tile {
        self.data[self.get_simple_idx(row, col)]
    }
}

pub fn solve(input: &str) -> Option<(u64, String)> {
    let mut commands = Vec::new();
    for data in input
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().ok()).collect_tuple())
    {
        let (row, col) = data?;
        commands.push((row?, col?));
    }
    let mut grid = Grid::new(commands);

    let part1 = grid.shortest_path()?;

    let indices: Vec<_> = (0..grid.remaining_commands.len()).collect();
    let part2_idx = indices
        .binary_search_by(|&idx| grid.first_failure(idx))
        .ok()?
        - 1; // Classic off-by-one error
    let part2 = *grid.remaining_commands.get(part2_idx)?;
    Some((part1, format!("{},{}", part2.0, part2.1)))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p18.txt");
    #[test]
    fn day18_solve() {
        dbg!(super::solve(INPUT));
    }
}
