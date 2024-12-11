#[derive(Debug, Clone, Copy, Hash)]
struct Stone {
    number: u64,
    quantity: u64,
}

impl Stone {
    fn new(number: u64, quantity: u64) -> Self {
        Self { number, quantity }
    }
    fn blink(&mut self) -> Option<Self> {
        if self.number == 0 {
            self.number = 1;
            return None;
        }
        let num_digits = self.number.ilog10() + 1;
        if num_digits % 2 == 0 {
            let pt = 10u64.pow(num_digits / 2);
            let out = Some(Stone::new(self.number % pt, self.quantity));
            self.number = self.number / pt;
            out
        } else {
            self.number *= 2024;
            None
        }
    }
}

fn compact(stones: &mut Vec<Stone>) {
    // The vast majority of the stones will end up being single digit, so compact them together
    let mut temp: Vec<Stone> = (0..10).map(|x| Stone::new(x, 0)).collect();
    for s in stones.iter() {
        if s.number < 10 {
            temp[s.number as usize].quantity += s.quantity;
        }
    }
    stones.retain(|s| s.number >= 10);
    stones.extend(temp.into_iter().filter(|s| s.quantity > 0));
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut remaining: Vec<_> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| Stone::new(x.parse().unwrap(), 1))
        .collect();

    let mut part1 = 0;
    for epoch in 0..75 {
        let len = remaining.len();
        for i in 0..len {
            let right = remaining[i].blink();
            if let Some(right) = right {
                remaining.push(right);
            }
        }
        compact(&mut remaining);
        if epoch == 24 {
            part1 = remaining.iter().map(|s| s.quantity).sum();
        }
    }
    Some((part1, remaining.iter().map(|s| s.quantity).sum()))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p11.txt");
    #[test]
    fn day11_solve() {
        dbg!(super::solve(INPUT));
    }
}
