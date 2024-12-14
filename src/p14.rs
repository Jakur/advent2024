use itertools::Itertools;
use std::collections::HashSet;

// const GRID_X_LENGTH: i64 = 11;
// const GRID_Y_LENGTH: i64 = 7;
const GRID_X_LENGTH: i64 = 101;
const GRID_Y_LENGTH: i64 = 103;

fn print_tree(vec: Vec<[bool; GRID_X_LENGTH as usize]>) {
    for arr in vec {
        for a in arr {
            if a {
                print!("*")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    const PART1_SECONDS: i64 = 100;
    let mut part1 = [0, 0, 0, 0];
    let mut max_score = 0;
    let mut part2 = 0;
    let mut score_buffer = [0; 10];
    let mut best_tree = Vec::new();
    let robots: Vec<_> = input
        .lines()
        .map(|line| {
            let (px, py, vx, vy) = line
                .split(['=', ',', ' '])
                .filter_map(|x| x.parse::<i64>().ok())
                .collect_tuple()
                .unwrap();
            Robot { px, py, vx, vy }
        })
        .collect();
    for seconds in 0..10000 {
        let mut points = HashSet::new();
        for robot in robots.iter() {
            // Set final position
            let mut x_pos = (robot.px + robot.vx * seconds) % GRID_X_LENGTH;
            let mut y_pos = (robot.py + robot.vy * seconds) % GRID_Y_LENGTH;
            if x_pos < 0 {
                x_pos = GRID_X_LENGTH + x_pos;
            }
            if y_pos < 0 {
                y_pos = GRID_Y_LENGTH + y_pos;
            }
            // Set part 1
            if seconds == PART1_SECONDS {
                if x_pos > GRID_X_LENGTH / 2 {
                    if y_pos > GRID_Y_LENGTH / 2 {
                        part1[0] += 1;
                    } else if y_pos < GRID_Y_LENGTH / 2 {
                        part1[1] += 1;
                    }
                } else if x_pos < GRID_X_LENGTH / 2 {
                    if y_pos > GRID_Y_LENGTH / 2 {
                        part1[2] += 1;
                    } else if y_pos < GRID_Y_LENGTH / 2 {
                        part1[3] += 1;
                    }
                }
            }
            points.insert((y_pos as usize, x_pos as usize));
        }
        let grid = Grid { data: points };
        let grid_score = grid.get_largest_comp();
        if grid_score > max_score {
            // I'm not positive if this indexing is right, but it works
            let mut data = vec![[false; GRID_X_LENGTH as usize]; GRID_Y_LENGTH as usize];
            for (y_pos, x_pos) in grid.data {
                data[y_pos][x_pos] = true;
            }
            max_score = grid_score;
            best_tree = data;
            part2 = seconds;
            // Early stop if maximum clump size is extremely anomalous
            let dev = std_smooth(&score_buffer);
            if max_score as f64 > 30.0 * dev {
                break;
            }
        }
        score_buffer[seconds as usize % score_buffer.len()] = grid_score;
    }
    print_tree(best_tree);
    Some((part1.into_iter().product(), part2 as u64))
}

/// Take the standard deviation of recent measurements, but add an offset so it is never too low
fn std_smooth(arr: &[u64]) -> f64 {
    const EPS: f64 = 2.0;
    let size = arr.len() as f64;
    let mean = arr.iter().sum::<u64>() as f64 / size;
    let variance = arr
        .iter()
        .map(|val| {
            let val = (*val as f64) - mean;
            val * val
        })
        .sum::<f64>()
        / size;
    (variance + EPS).sqrt()
}

#[derive(Debug)]
struct Grid {
    data: HashSet<(usize, usize)>,
}
impl Grid {
    /// Returns the size of the largest clump of robots
    fn get_largest_comp(&self) -> u64 {
        let mut global_visited = HashSet::new();
        let mut largest_comp = 0;
        for tuple in self.data.iter() {
            if !global_visited.contains(tuple) {
                let (row, col) = *tuple;
                let starting_len = global_visited.len() as u64;
                self.dfs((row, col), &mut global_visited);
                let comp_size = (global_visited.len() as u64) - starting_len;
                largest_comp = std::cmp::max(largest_comp, comp_size);
            }
        }
        largest_comp
    }
    fn dfs(&self, position: (usize, usize), visited: &mut HashSet<(usize, usize)>) {
        visited.insert(position);
        let (row, col) = position;
        let neighbors = [
            (row - 1, col),
            (row, col + 1),
            (row + 1, col),
            (row, col - 1),
        ];
        for n in neighbors {
            let (row, col) = n;
            if self.get(row, col) {
                if !visited.contains(&(row, col)) {
                    self.dfs(n, visited);
                }
            }
        }
    }
    fn get(&self, row: usize, col: usize) -> bool {
        self.data.contains(&(row, col))
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p14.txt");
    #[test]
    fn day14_solve() {
        dbg!(super::solve(INPUT));
    }
}
