fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn mul(a: i64, b: i64) -> i64 {
    a * b
}

fn cat(a: i64, b: i64) -> i64 {
    let b_digits = b.ilog10() + 1;
    a * 10i64.pow(b_digits) + b
}

#[derive(Debug)]
struct Equation {
    goal: i64,
    numbers: Vec<i64>,
}

impl Equation {
    /// Part 1 operations only, add and mul
    fn evaluate(&self, idx: usize, acc: i64) -> bool {
        if idx >= self.numbers.len() {
            return acc == self.goal;
        }
        let next = self.numbers[idx];
        for op in [add, mul] {
            let temp = op(acc, next);
            if self.evaluate(idx + 1, temp) {
                return true;
            }
        }
        false
    }
    /// Added concatenation from Part 2
    fn evaluate2(&self, idx: usize, acc: i64) -> bool {
        if idx >= self.numbers.len() {
            return acc == self.goal;
        }
        let next = self.numbers[idx];
        for op in [cat, add, mul] {
            let temp = op(acc, next);
            if self.evaluate2(idx + 1, temp) {
                return true;
            }
        }
        false
    }
}

pub fn solve(input: &str) -> Option<(i64, i64)> {
    let equations = input.lines().map(|line| {
        let mut split = line
            .split(&[':', ' '])
            .filter_map(|x| x.parse::<i64>().ok());
        let goal = split.next().unwrap();
        let numbers: Vec<_> = split.collect();
        assert!(numbers.len() >= 2);
        Equation { goal, numbers }
    });
    let mut part1 = 0;
    let mut part2 = 0;
    for eq in equations {
        if eq.evaluate(1, eq.numbers[0]) {
            part1 += eq.goal;
        } else if eq.evaluate2(1, eq.numbers[0]) {
            part2 += eq.goal;
        }
    }
    part2 += part1;
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p7.txt");
    #[test]
    fn day7_solve() {
        dbg!(super::solve(INPUT));
    }
}
