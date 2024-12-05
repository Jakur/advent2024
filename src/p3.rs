use std::ops::Range;

use regex::Regex;

#[derive(Debug)]
struct Command {
    location: usize,
    do_command: bool,
}

impl Command {
    fn new(location: usize, do_command: bool) -> Self {
        Self {
            location,
            do_command,
        }
    }
}

#[derive(Debug)]
struct Commands {
    data: Vec<Command>,
    ptr: usize,
}

impl Commands {
    fn get_range(&self) -> Range<usize> {
        let st = self.data[self.ptr].location;
        let end = self
            .data
            .get(self.ptr + 1)
            .map(|x| x.location)
            .unwrap_or(usize::MAX);
        st..end
    }
    fn should_compute(&mut self, start: usize) -> bool {
        if start < self.data[0].location {
            return true;
        }
        let mut range = self.get_range();
        while !range.contains(&start) {
            self.ptr += 1;
            range = self.get_range();
        }
        self.data[self.ptr].do_command
    }
}

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let switch = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let commands = switch
        .captures_iter(input)
        .map(|x| {
            let m = x.iter().next().flatten().unwrap(); // No sub-matches possible
            Command::new(m.end(), m.as_str() == "do()")
        })
        .collect();
    let mut commands = Commands {
        data: commands,
        ptr: 0,
    };
    for val in re.captures_iter(input) {
        let prod: i32 = val
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
            .product();
        let loc = val.get(0).unwrap().start();
        part1 += prod;
        if commands.should_compute(loc) {
            part2 += prod;
        }
    }
    Some((part1, part2))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p3.txt");
    #[test]
    fn day3_solve() {
        dbg!(super::solve(INPUT));
    }
}
