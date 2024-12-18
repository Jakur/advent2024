use super::Direction;
use std::collections::{HashMap, HashSet};

type Fences = [bool; 4];

#[derive(Debug)]
struct SquareGrid {
    data: Vec<u8>,
    length: usize,
}
impl SquareGrid {
    fn new(data: Vec<u8>, length: usize) -> Self {
        Self { data, length }
    }
    /// Returns score for both parts
    fn score(&self) -> (u64, u64) {
        let mut global_visited = HashSet::new();
        let mut cost = 0;
        let mut cost2 = 0;
        for row in 0..self.length {
            for col in 0..self.length {
                if !global_visited.contains(&(row, col)) {
                    let mut visited = HashSet::new();
                    let val = self.get(row, col).unwrap();
                    let perimeter = self.dfs(val, (row, col), &mut visited);
                    let area = (visited.len()) as u64;
                    let sides = self.get_sides(val, &visited);
                    global_visited.extend(visited.into_iter());
                    cost += perimeter * area;
                    cost2 += sides * area;
                }
            }
        }
        (cost, cost2)
    }
    fn get_sides(&self, token: u8, visited: &HashSet<(usize, usize)>) -> u64 {
        let mut marks: HashMap<(usize, usize), Fences> = visited
            .iter()
            .copied()
            .map(|position| (position, [false, false, false, false]))
            .collect();
        let mut sides = 0;
        for &(row, col) in visited {
            let directions = [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ];
            let fences = marks.get(&(row, col)).unwrap().clone();
            for direction in directions.iter().copied() {
                if fences[direction as usize] {
                    continue; // Already built the fence
                }
                let n = direction.get_square((row, col));
                if self.get(n.0, n.1) != Some(token) {
                    // Needs a fence
                    sides += 1;
                    // Raycast perpendicular to set fence status on neighbors
                    let (perp1, perp2) = direction.get_perp();
                    for perp in [perp1, perp2] {
                        let mut square = (row, col);
                        while let Some(m) = marks.get_mut(&square) {
                            m[direction as usize] = true;
                            square = perp.get_square(square);
                            // Ensure square needs this fence too, i.e. the fence would be contiguous
                            let n = direction.get_square(square);
                            if self.get(n.0, n.1) == Some(token) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        sides
    }
    fn dfs(
        &self,
        goal: u8,
        position: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> u64 {
        visited.insert(position);
        let (row, col) = position;
        let neighbors = [
            (row - 1, col),
            (row, col + 1),
            (row + 1, col),
            (row, col - 1),
        ];
        let mut perimeter = 4;
        for n in neighbors {
            let (row, col) = n;
            if let Some(sq) = self.get(row, col) {
                if sq == goal {
                    perimeter -= 1; // They share an "edge"
                    if !visited.contains(&(row, col)) {
                        perimeter += self.dfs(goal, n, visited);
                    }
                }
            }
        }
        perimeter
    }
    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if row >= self.length || col >= self.length {
            None
        } else {
            Some(self.data[row * self.length + col])
        }
    }
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut vec = Vec::new();
    let mut length = 0;
    for line in input.lines() {
        for b in line.as_bytes() {
            vec.push(*b);
        }
        length += 1;
    }
    assert_eq!(length * length, vec.len()); // Square grid;
    let grid = SquareGrid::new(vec, length);
    Some(grid.score())
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p12.txt");
    #[test]
    fn day12_solve() {
        dbg!(super::solve(INPUT));
    }
}
