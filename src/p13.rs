use itertools::Itertools;
use num_rational::Rational64;
use regex::Regex;

const PART2_OFFSET: i64 = 10_000_000_000_000;

struct Equation {
    a1: i64,
    b1: i64,
    a2: i64,
    b2: i64,
    eqs1: i64,
    eqs2: i64,
}

impl Equation {
    fn solve_tokens(&self) -> Option<i64> {
        // Solve a system of 2 equations
        let elim_a = Rational64::new(-1 * self.a1, self.a2);
        let bs = elim_a * self.b2 + self.b1;
        let eqs = elim_a * self.eqs2 + self.eqs1;
        let b_sol = eqs / bs;
        if b_sol.is_integer() {
            // Working, now plug into equation 1
            let a_sol = (b_sol * (-1 * self.b1) + self.eqs1) / self.a1;
            if a_sol.is_integer() {
                // 3a + b
                Some(a_sol.numer() * 3 + b_sol.numer())
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn get_ints(re: &Regex, line: &str) -> Option<(i64, i64)> {
    let captures = re.captures(line)?;
    captures
        .iter()
        .skip(1) // 0th is the full string match
        .filter_map(|x| x.map(|x| x.as_str().parse::<i64>().ok()))
        .flatten()
        .collect_tuple()
}

pub fn solve(input: &str) -> Option<(i64, i64)> {
    // No capture non-numeric, capture number--repeated twice
    let re = Regex::new(r"(?:[^\d]+)(\d+)(?:[^\d]+)(\d+)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    for lines in input
        .lines()
        .filter(|x| !x.is_empty())
        .chunks(3)
        .into_iter()
    {
        let (line1, line2, line3) = lines.collect_tuple()?;
        let (a1, a2) = get_ints(&re, &line1)?;
        let (b1, b2) = get_ints(&re, &line2)?;
        let (eqs1, eqs2) = get_ints(&re, &line3)?;
        let equation = Equation {
            a1,
            a2,
            b1,
            b2,
            eqs1,
            eqs2,
        };
        if let Some(token_count) = equation.solve_tokens() {
            part1 += token_count;
        }
        let equation2 = Equation {
            eqs1: eqs1 + PART2_OFFSET,
            eqs2: eqs2 + PART2_OFFSET,
            ..equation
        };
        if let Some(token_count) = equation2.solve_tokens() {
            part2 += token_count;
        }
    }
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p13.txt");
    #[test]
    fn day13_solve() {
        dbg!(super::solve(INPUT));
    }
}
