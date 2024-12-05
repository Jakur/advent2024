struct SquareGrid {
    data: Vec<char>,
    length: usize,
}

impl SquareGrid {
    fn new(data: Vec<char>, length: usize) -> Self {
        Self { data, length }
    }
    fn cross(&self, row: usize, col: usize) -> bool {
        if self.get(row, col).unwrap() != 'A' {
            return false;
        }
        let c1 = &[(row - 1, col - 1), (row, col), (row + 1, col + 1)];
        let c2 = &[(row - 1, col + 1), (row, col), (row + 1, col - 1)];
        let mut count = 0;
        for word in [self.build_string(c1), self.build_string(c2)] {
            if let Some(word) = word {
                if word == "MAS" || word == "SAM" {
                    count += 1;
                }
            }
        }
        count == 2
    }
    fn hits(&self, row: usize, col: usize) -> i32 {
        let root = self.get(row, col).unwrap();
        if (root != 'S') && (root != 'X') {
            return 0;
        }
        let diag = &[
            (row, col),
            (row + 1, col + 1),
            (row + 2, col + 2),
            (row + 3, col + 3),
        ];
        let diag2 = &[
            (row, col),
            (row + 1, col - 1),
            (row + 2, col - 2),
            (row + 3, col - 3),
        ];
        let vert = &[(row, col), (row + 1, col), (row + 2, col), (row + 3, col)];
        let hor = &[(row, col), (row, col + 1), (row, col + 2), (row, col + 3)];
        let words = [
            self.build_string(diag),
            self.build_string(diag2),
            self.build_string(hor),
            self.build_string(vert),
        ];
        let mut hits = 0;
        for word in words {
            if let Some(word) = word {
                let word = word.as_str();
                if word == "XMAS" || word == "SAMX" {
                    hits += 1;
                }
            }
        }
        hits
    }
    fn build_string(&self, arr: &[(usize, usize)]) -> Option<String> {
        arr.iter().map(|&(r, c)| self.get(r, c)).collect()
    }
    fn get(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.length || col >= self.length {
            None
        } else {
            Some(self.data[row * self.length + col])
        }
    }
}

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let length = input
        .lines()
        .nth(0)?
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .count();
    let data = input.chars().filter(|x| x.is_ascii_alphabetic()).collect();
    let grid = SquareGrid::new(data, length);
    assert_eq!(length * length, grid.data.len());
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..length {
        for j in 0..length {
            part1 += grid.hits(i, j);
            part2 += grid.cross(i, j) as i32;
        }
    }
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p4.txt");
    #[test]
    fn day4_solve() {
        dbg!(super::solve(INPUT));
    }
}
