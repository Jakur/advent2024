#[derive(Debug, Clone, Copy, PartialEq)]
enum Marker {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

enum Status {
    Running,
    OutofBounds,
    Cycle,
}

struct SquareGrid {
    data: Vec<Marker>,
    length: usize,
    start: (usize, usize),
    position: (usize, usize),
    start_direction: Direction,
    direction: Direction,
    history: Vec<usize>,
}
impl SquareGrid {
    fn step(&mut self) -> Status {
        let (row, col) = self.position;
        let location = match self.direction {
            Direction::North => (row - 1, col),
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
        };
        if let Some(val) = self.get_mut(location.0, location.1) {
            if let Marker::Obstacle = val {
                // Turn, do not move
                self.direction = self.direction.turn();
                self.history.push(location.0 * self.length + location.1);
                let hist_len = self.history.len();
                if hist_len > 10 {
                    // Check for cycles. See if the same two consecutive obstacle turns are observed twice
                    let last_pair = (self.history[hist_len - 2], self.history[hist_len - 1]);
                    for (left, right) in self
                        .history
                        .iter()
                        .rev()
                        .skip(3)
                        .zip(self.history.iter().rev().skip(2))
                    {
                        if *left == last_pair.0 && *right == last_pair.1 {
                            return Status::Cycle;
                        }
                    }
                }
            } else {
                *val = Marker::Visited;
                self.position = location;
            }
            Status::Running
        } else {
            Status::OutofBounds
        }
    }
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Marker> {
        if row >= self.length || col >= self.length {
            None
        } else {
            Some(&mut self.data[row * self.length + col])
        }
    }
    fn reset(&mut self) {
        self.position = self.start;
        self.direction = self.start_direction;
        self.history.clear();
    }
}

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let mut part2 = 0;
    let mut direction = None;
    let mut data = Vec::new();
    let mut counter = 0;
    let mut start = (0, 0);
    for (row, line) in input.lines().enumerate() {
        counter = 0;
        for (col, c) in line.chars().enumerate() {
            counter += 1;
            let mut value = Marker::Empty;
            match c {
                '.' => {}
                '#' => value = Marker::Obstacle,
                '^' => {
                    value = Marker::Visited;
                    direction = Some(Direction::North);
                    start = (row, col);
                }
                _ => unimplemented!(), // Seems it always starts north?
            };
            data.push(value);
        }
    }
    let direction = direction?;
    assert_eq!(counter * counter, data.len()); // Grid is square
    let mut grid = SquareGrid {
        data,
        length: counter,
        position: start,
        start,
        direction,
        start_direction: direction,
        history: Vec::new(),
    };
    while let Status::Running = grid.step() {}
    let start_idx = grid.start.0 * grid.length + grid.start.1;
    let marked: Vec<_> = grid
        .data
        .iter()
        .copied()
        .enumerate()
        .filter(|(idx, x)| (*x == Marker::Visited) && (*idx != start_idx))
        .collect();
    let part1 = marked.len() + 1; // Marked + start position

    // Only need to check marked squares, otherwise the extra obstacle will never be encountered
    for (idx, _) in marked.into_iter() {
        grid.reset();
        grid.data[idx] = Marker::Obstacle;
        let mut status = Status::Running;
        while let Status::Running = status {
            status = grid.step();
        }
        if let Status::Cycle = status {
            part2 += 1;
        }
        grid.data[idx] = Marker::Visited;
    }
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p6.txt");
    #[test]
    fn day6_solve() {
        dbg!(super::solve(INPUT));
    }
}
