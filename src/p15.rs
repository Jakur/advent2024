use super::Direction;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Robot,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
}

impl Tile {
    fn neighbor_direction(&self) -> Direction {
        if let Self::BoxLeft = self {
            Direction::East
        } else {
            Direction::West
        }
    }
}

struct Grid {
    data: Vec<Tile>,
    num_rows: usize,
    num_cols: usize,
    robot_row: usize,
    robot_col: usize,
}
impl Grid {
    fn new(data: Vec<Tile>, num_rows: usize) -> Self {
        let num_cols = data.len() / num_rows;
        let mut out = Self {
            data,
            num_rows,
            num_cols,
            robot_row: 0,
            robot_col: 0,
        };
        let (robot_row, robot_col) = out.find_robot();
        out.robot_row = robot_row;
        out.robot_col = robot_col;
        out
    }
    fn find_robot(&self) -> (usize, usize) {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                if let Tile::Robot = self.get(row, col) {
                    return (row, col);
                }
            }
        }
        unimplemented!()
    }
    fn score_boxes(&self) -> u64 {
        let mut out = 0;
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                // The left side of the box is always closer to the left side, and equally close to the top
                if let Tile::Box | Tile::BoxLeft = self.get(row, col) {
                    out += 100 * row + col;
                }
            }
        }
        out as u64
    }
    fn robot_move(&mut self, dir: Direction) {
        if self.push(self.robot_row, self.robot_col, dir) {
            // Update robot location cache
            let (row, col) = dir.get_square((self.robot_row, self.robot_col));
            self.robot_row = row;
            self.robot_col = col;
        }
    }
    /// Checks if the push is valid without executing it. Necessary for Part 2
    fn can_push(&self, row: usize, col: usize, dir: Direction) -> bool {
        let (next_row, next_col) = dir.get_square((row, col));
        let next_tile = self.get(next_row, next_col);
        let can_push = match next_tile {
            Tile::Robot => unimplemented!(),
            Tile::Wall => false,
            Tile::Box => self.can_push(next_row, next_col, dir),
            Tile::Empty => true,
            Tile::BoxLeft | Tile::BoxRight => {
                match dir {
                    Direction::East | Direction::West => self.can_push(next_row, next_col, dir), // Easy case
                    _ => {
                        let neighbor_direction = next_tile.neighbor_direction();
                        // Find the other half of the box
                        let (neighbor_row, neighbor_col) =
                            neighbor_direction.get_square((next_row, next_col));
                        // And make sure both halves are pushable
                        let pushable = self.can_push(next_row, next_col, dir)
                            && self.can_push(neighbor_row, neighbor_col, dir);
                        pushable
                    }
                }
            }
        };
        can_push
    }
    /// Recursively executes a push if it is valid
    fn push(&mut self, row: usize, col: usize, dir: Direction) -> bool {
        let tile = self.get(row, col);
        let (next_row, next_col) = dir.get_square((row, col));
        let next_tile = self.get(next_row, next_col);
        let can_push = match next_tile {
            Tile::Robot => unimplemented!(),
            Tile::Wall => false,
            Tile::Box => self.push(next_row, next_col, dir),
            Tile::Empty => true,
            Tile::BoxLeft | Tile::BoxRight => {
                match dir {
                    Direction::East | Direction::West => self.push(next_row, next_col, dir), // Easy case
                    _ => {
                        let neighbor_direction = next_tile.neighbor_direction();
                        // Find the other half of the box
                        let (neighbor_row, neighbor_col) =
                            neighbor_direction.get_square((next_row, next_col));
                        // And make sure both halves are pushable
                        // Note, you must not execute the first branch until you are sure the second branch is okay
                        let pushable = self.can_push(next_row, next_col, dir)
                            && self.push(neighbor_row, neighbor_col, dir);
                        if pushable {
                            self.push(next_row, next_col, dir); // Go back and execute the first branch
                        }
                        pushable
                    }
                }
            }
        };
        if can_push {
            *self.get_mut(next_row, next_col) = tile;
            *self.get_mut(row, col) = Tile::Empty;
        }
        can_push
    }
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Tile {
        &mut self.data[row * self.num_cols + col]
    }
    fn get(&self, row: usize, col: usize) -> Tile {
        self.data[row * self.num_cols + col]
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let s = match self.get(row, col) {
                    Tile::Robot => "@",
                    Tile::Wall => "#",
                    Tile::Box => "O",
                    Tile::Empty => ".",
                    Tile::BoxLeft => "[",
                    Tile::BoxRight => "]",
                };
                write!(f, "{}", s)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut data1 = Vec::new();
    let mut data2 = Vec::new();
    let mut num_lines = 0;
    let mut lines = input.lines();
    for line in lines.take_while_ref(|line| !line.is_empty()) {
        for (tile1, tile2, tile3) in line.chars().map(|c| match c {
            '.' => (Tile::Empty, Tile::Empty, Tile::Empty),
            '@' => (Tile::Robot, Tile::Robot, Tile::Empty),
            '#' => (Tile::Wall, Tile::Wall, Tile::Wall),
            'O' => (Tile::Box, Tile::BoxLeft, Tile::BoxRight),
            _ => unimplemented!(),
        }) {
            // Part 1 Parse
            data1.push(tile1);
            // Part 2 Parse
            data2.push(tile2);
            data2.push(tile3);
        }
        num_lines += 1;
    }
    let mut commands = Vec::new();
    for line in lines {
        for command in line.chars() {
            let command = match command {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => return None,
            };
            commands.push(command);
        }
    }

    let mut grid = Grid::new(data1, num_lines);
    let mut grid2 = Grid::new(data2, num_lines);
    for command in commands {
        grid.robot_move(command);
        grid2.robot_move(command);
    }
    Some((grid.score_boxes(), grid2.score_boxes()))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p15.txt");
    #[test]
    fn day15_solve() {
        dbg!(super::solve(INPUT));
    }
}
