#[derive(Debug)]
struct SquareGrid {
    data: Vec<u8>,
    length: usize,
}
impl SquareGrid {
    fn new(data: Vec<u8>, length: usize) -> Self {
        Self { data, length }
    }
    /// Returns scores for part1 and part2, respectively
    fn score(&self, idx: usize) -> (usize, usize) {
        let mut buffer = Vec::new();
        let position = (idx / self.length, idx % self.length);
        let num_trails = self.dfs(1, position, &mut buffer);
        // Achieve set-like behavior
        buffer.sort();
        buffer.dedup();
        (buffer.len(), num_trails)
    }
    fn dfs(&self, goal: u8, position: (usize, usize), data: &mut Vec<(usize, usize)>) -> usize {
        if goal == 10 {
            data.push(position); // Already checked correctness in previous parent function call
            return 1;
        }
        let (row, col) = position;
        let neighbors = [
            (row - 1, col),
            (row, col + 1),
            (row + 1, col),
            (row, col - 1),
        ];
        let mut num_trails = 0;
        for n in neighbors {
            let (row, col) = n;
            if let Some(sq) = self.get(row, col) {
                if sq == goal {
                    num_trails += self.dfs(goal + 1, n, data);
                }
            }
        }
        return num_trails;
    }
    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if row >= self.length || col >= self.length {
            None
        } else {
            Some(self.data[row * self.length + col])
        }
    }
}

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let mut vec = Vec::new();
    let mut length = 0;
    for line in input.lines() {
        for b in line.as_bytes() {
            vec.push(*b - b'0');
        }
        length += 1;
    }
    assert_eq!(length * length, vec.len()); // Square grid;
    let grid = SquareGrid::new(vec, length);
    let mut part1 = 0;
    let mut part2 = 0;
    for (p1, p2) in
        grid.data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if *v == 0 { Some(grid.score(i)) } else { None })
    {
        part1 += p1;
        part2 += p2;
    }
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p10.txt");
    #[test]
    fn day10_solve() {
        dbg!(super::solve(INPUT));
    }
}
