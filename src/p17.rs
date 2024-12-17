use std::fmt::Write;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Operation {
    code: OpCode,
    arg: u8,
}

impl Operation {
    fn new(val1: u8, val2: u8) -> Self {
        use OpCode::*;
        let code = match val1 {
            0 => ADV,
            1 => BXL,
            2 => BST,
            3 => JNZ,
            4 => BXC,
            5 => OUT,
            6 => BDV,
            7 => CDV,
            _ => unimplemented!(),
        };
        Self { code, arg: val2 }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum OpCode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

#[derive(Debug)]
struct Machine {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ptr: usize,
    code: Vec<u8>,
    out_buffer: Vec<u8>,
}

impl Machine {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64, code: Vec<u8>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            ptr: 0,
            code,
            out_buffer: Vec::new(),
        }
    }
    fn combo(&self, val: u8) -> u64 {
        match val {
            0..=3 => val as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unimplemented!(),
        }
    }
    /// Execute the program and return its string output for part 1
    fn run(&mut self) -> Option<String> {
        let mut out = String::new();
        while self.step().is_some() {}
        for i in self.out_buffer[0..self.out_buffer.len() - 1]
            .iter()
            .copied()
        {
            write!(&mut out, "{},", i).ok()?;
        }
        write!(&mut out, "{}", self.out_buffer.last()?).ok()?;
        Some(out)
    }
    /// Look for the valid input with backtracking, and abort early if a solution cannot become valid
    fn recurse(&mut self, start_val: u8, high_bits: u64, num_steps: u8) -> Option<u64> {
        if num_steps >= self.code.len() as u8 {
            return Some(high_bits);
        }
        for num in start_val..8 {
            let reg_a = high_bits << 3 | num as u64;
            // If this configuration doesn't work already, it will never work down the line, continue
            if !self.is_valid(reg_a, num_steps + 1) {
                continue;
            }
            // If we found a valid solution, we're done
            if let Some(value) = self.recurse(0, reg_a, num_steps + 1) {
                return Some(value);
            }
        }
        None
    }
    fn search(&mut self) -> Option<u64> {
        let blocks: Vec<_> = self
            .code
            .chunks_exact(2)
            .map(|s| Operation::new(s[0], s[1]))
            .collect();
        // Check preconditions
        // Only jump is at the end of the program, and jumps on A != 0 to 0
        let len = blocks.len();
        assert_eq!(blocks.last(), Some(&Operation::new(3, 0)));
        assert!(blocks
            .iter()
            .take(len - 1)
            .find(|b| b.code == OpCode::JNZ)
            .is_none());
        // Now we know that our program does not do anything crazy
        // It is a do while loop with no other control flow
        // Note only instruction ADV can modify the A register (which is the loop variable)
        let adv: Vec<_> = blocks.iter().filter(|b| b.code == OpCode::ADV).collect();
        assert_eq!(adv.len(), 1); // Only one update to A per iteration of the loop
        let div: u64 = 1 << adv[0].arg;
        assert!(div == 8); // Bottom 3 bits
        let output_length = self.code.len() as u32;
        let a_lower_bound = div.pow(output_length - 1);
        let a_upper_bound = div.pow(output_length);

        // Build up by building the up the top 3 bits of A iteratively
        let part2 = self.recurse(1, 0, 0); // The first bit cannot be 0
        if let Some(part2) = part2 {
            assert!(part2 >= a_lower_bound && part2 <= a_upper_bound);
        }
        part2
    }
    fn is_valid(&mut self, reg_a: u64, num_steps: u8) -> bool {
        self.set_state(reg_a);
        while self.step().is_some() {}
        let out = self.check_output_tail(num_steps);
        out
    }
    /// Check that the last num_steps elements of the output buffer match the program data
    fn check_output_tail(&mut self, num_steps: u8) -> bool {
        let cond = self.out_buffer.len() == num_steps as usize;
        cond && self
            .out_buffer
            .drain(..)
            .rev()
            .zip(self.code.iter().skip(1).rev())
            .all(|(a, b)| a == *b)
    }
    fn set_state(&mut self, reg_a: u64) {
        // State of B and C should be irrelevant
        self.out_buffer.clear();
        self.reg_a = reg_a;
        self.ptr = 0;
    }
    fn step(&mut self) -> Option<()> {
        let x1 = *self.code.get(self.ptr)?;
        let x2 = *self.code.get(self.ptr + 1)?;
        let op = Operation::new(x1, x2);

        match op.code {
            OpCode::ADV => {
                let denom = 1 << self.combo(x2);
                self.reg_a /= denom;
            }
            OpCode::BXL => self.reg_b ^= x2 as u64,
            OpCode::BST => self.reg_b = self.combo(x2) % 8,
            OpCode::JNZ => {
                if self.reg_a != 0 {
                    self.ptr = x2 as usize;
                    return Some(()); // Do not move instruction ptr further
                }
            }
            OpCode::BXC => self.reg_b ^= self.reg_c,
            OpCode::OUT => self.out_buffer.push((self.combo(x2) % 8) as u8),
            OpCode::BDV => {
                let denom = 1 << self.combo(x2);
                self.reg_b = self.reg_a / denom;
            }
            OpCode::CDV => {
                let denom = 1 << self.combo(x2);
                self.reg_c = self.reg_a / denom;
            }
        }
        self.ptr += 2;
        Some(())
    }
}

pub fn solve(input: &str) -> Option<(String, u64)> {
    let (reg_a, reg_b, reg_c) = input
        .lines()
        .take(3)
        .map(|line| line.split_whitespace().last().unwrap().parse().ok())
        .collect_tuple()?;

    let code: Option<Vec<u8>> = input
        .lines()
        .nth(4)?
        .split_whitespace()
        .nth(1)?
        .split(",")
        .map(|x| x.parse().ok())
        .collect();
    let mut machine = Machine::new(reg_a?, reg_b?, reg_c?, code?);
    let part1 = machine.run()?;
    let part2 = machine.search()?;
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p17.txt");
    #[test]
    fn day17_solve() {
        dbg!(super::solve(INPUT));
    }
}
