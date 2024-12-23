use itertools::Itertools;

use crate::Direction;
use std::collections::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum PadVal {
    Empty = 0,
    Up,
    A,
    Left,
    Down,
    Right,
}

impl PadVal {
    fn step(self, origin: (usize, usize)) -> (usize, usize) {
        match self {
            PadVal::Empty => unimplemented!(),
            PadVal::Up => Direction::North.get_square(origin),
            PadVal::A => unimplemented!(),
            PadVal::Left => Direction::West.get_square(origin),
            PadVal::Down => Direction::South.get_square(origin),
            PadVal::Right => Direction::East.get_square(origin),
        }
    }
}

impl std::fmt::Debug for PadVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Up => write!(f, "^"),
            Self::A => write!(f, "A"),
            Self::Left => write!(f, "<"),
            Self::Down => write!(f, "v"),
            Self::Right => write!(f, ">"),
        }
    }
}

#[derive(Debug)]
struct NumPad {
    data: [[u8; 3]; 4],
    lookup: [(usize, usize); 11],
    location: u8,
    parent: DirPad,
}

impl NumPad {
    fn new(parent: DirPad) -> Self {
        let data = [[7, 8, 9], [4, 5, 6], [1, 2, 3], [255, 0, 10]];
        let mut lookup = [Default::default(); 11];
        for i in 0..4 {
            for j in 0..3 {
                let val = data[i][j];
                if val != 255 {
                    lookup[val as usize] = (i, j);
                }
            }
        }
        Self {
            data,
            lookup: lookup,
            location: 10,
            parent,
        }
    }
    fn next_number(&mut self, goal: u8) -> u64 {
        // dbg!(goal);
        let paths = self.iterate_paths(self.location, goal);
        let mut best_cost = usize::MAX;
        // dbg!(paths.len());
        for path in paths {
            let p = self.parent.walk(path);
            if p.absolute_path.len() < best_cost {
                best_cost = p.absolute_path.len();
                // dbg!(&p);
            }
        }
        self.location = goal;
        best_cost as u64
    }
    fn reset(&mut self) {
        self.location = 10;
    }
}

impl KeyPad for NumPad {
    type Idx = u8;

    fn hole() -> (usize, usize) {
        (3, 0)
    }

    fn lookup(&self, idx: Self::Idx) -> (usize, usize) {
        self.lookup[idx as usize]
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CacheKey {
    start: PadVal,
    target: PadVal,
}

impl CacheKey {
    fn new(start: PadVal, target: PadVal) -> Self {
        Self { start, target }
    }
}

#[derive(Clone)]
struct CacheState {
    data: Vec<PadVal>,
    absolute_path: Vec<PadVal>,
}

impl CacheState {
    fn from_vec(vec: Vec<PadVal>, absolute_path: Vec<PadVal>) -> Self {
        Self {
            data: vec,
            absolute_path,
        }
    }
}

impl std::fmt::Debug for CacheState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path: ")?;
        for d in self.data.iter() {
            write!(f, "{:?}", *d)?;
        }
        writeln!(f)?;
        write!(f, "Absolute Path: ")?;
        for d in self.absolute_path.iter() {
            write!(f, "{:?}", *d)?;
        }
        writeln!(f)
    }
}

impl std::default::Default for CacheState {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            absolute_path: Vec::new(),
        }
    }
}

trait KeyPad {
    type Idx;
    fn hole() -> (usize, usize);
    fn lookup(&self, idx: Self::Idx) -> (usize, usize);
    fn iterate_paths(&mut self, st: Self::Idx, end: Self::Idx) -> Vec<Vec<PadVal>> {
        let st = self.lookup(st);
        let end = self.lookup(end);
        let steps = get_steps(st, end);
        let len = steps.len();
        let mut out = Vec::new();
        'outer: for mut perm in steps.into_iter().permutations(len) {
            // Check bounds
            let mut next = st;
            for s in perm.iter() {
                next = s.step(next);
                if next == Self::hole() {
                    continue 'outer;
                }
            }
            perm.push(PadVal::A); // Always ends with A?
            out.push(perm);
        }
        out
    }
}

impl KeyPad for DirPad {
    type Idx = PadVal;

    fn hole() -> (usize, usize) {
        (0, 0)
    }

    fn lookup(&self, idx: Self::Idx) -> (usize, usize) {
        self.lookup[idx as usize]
    }
}
fn get_steps(st: (usize, usize), end: (usize, usize)) -> Vec<PadVal> {
    let mut steps = Vec::new();
    if st.0 > end.0 {
        let add_num = st.0 - end.0;
        for _ in 0..add_num {
            steps.push(PadVal::Up);
        }
    } else if st.0 < end.0 {
        let add_num = end.0 - st.0;
        for _ in 0..add_num {
            steps.push(PadVal::Down);
        }
    }
    if st.1 > end.1 {
        let add_num = st.1 - end.1;
        for _ in 0..add_num {
            steps.push(PadVal::Left);
        }
    } else if st.1 < end.1 {
        let add_num = end.1 - st.1;
        for _ in 0..add_num {
            steps.push(PadVal::Right);
        }
    }
    steps
}

#[derive(Debug, Clone)]
struct DirPad {
    data: [[PadVal; 3]; 2],
    lookup: [(usize, usize); 6],
    cache: HashMap<CacheKey, CacheState>,
    location: PadVal,
    parent: Option<Box<Self>>,
}

impl DirPad {
    fn new(parent: Option<Box<Self>>) -> Self {
        let data = [
            [PadVal::Empty, PadVal::Up, PadVal::A],
            [PadVal::Left, PadVal::Down, PadVal::Right],
        ];
        let mut lookup = [Default::default(); 6];
        for i in 0..2 {
            for j in 0..3 {
                let val = data[i][j];
                lookup[val as usize] = (i, j);
            }
        }
        Self {
            data,
            lookup,
            cache: HashMap::new(),
            location: PadVal::A,
            parent,
        }
    }
    fn walk(&mut self, vec: Vec<PadVal>) -> CacheState {
        let mut state = self.location;
        let mut absolute = Vec::new();
        for val in vec.iter().copied() {
            let path = self.get_path(state, val);
            absolute.extend(path.absolute_path.iter().copied());
            state = val;
        }
        assert_eq!(state, PadVal::A);
        CacheState::from_vec(vec, absolute)
    }
    fn get_path(&mut self, st: PadVal, end: PadVal) -> &CacheState {
        let key = CacheKey::new(st, end);
        if st == end {
            return self
                .cache
                .entry(key)
                .or_insert_with(|| CacheState::from_vec(vec![PadVal::A], vec![PadVal::A]));
        }
        if self.cache.contains_key(&key) {
            return self.cache.get(&key).unwrap();
        }
        let all_paths = self.iterate_paths(st, end);
        let mut best_path = CacheState::default();
        let mut min_cost = usize::MAX;
        for path in all_paths {
            if let Some(parent) = self.parent.as_mut() {
                let check = parent.walk(path);
                if check.absolute_path.len() < min_cost {
                    min_cost = check.absolute_path.len();
                    best_path = check;
                }
            } else {
                if path.len() < min_cost {
                    min_cost = path.len();
                    best_path = CacheState::from_vec(path.clone(), path);
                }
            }
        }
        self.cache.entry(key).or_insert(best_path)
    }
}

fn build_numpad(depth: usize) -> NumPad {
    let mut root = DirPad::new(None);
    for _ in 0..depth - 1 {
        root = DirPad::new(Some(Box::new(root)));
    }
    NumPad::new(root)
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut numpad = build_numpad(2);
    let mut part1 = 0;
    for line in input.lines() {
        let seq = line.chars().map(|x| x.to_digit(16).unwrap() as u8);
        let numeric: String = line.chars().filter(|x| x.is_ascii_digit()).collect();
        let numeric_num = u64::from_str_radix(&numeric, 10).ok()?;
        let mut cost = 0;
        for dest in seq {
            cost += numpad.next_number(dest);
        }
        dbg!(cost);
        part1 += cost * numeric_num;
        numpad.reset();
    }
    dbg!(&numpad.parent.cache.len());
    dbg!(&numpad.parent.cache);
    // dbg!(&numpad.parent.parent.unwrap().parent.unwrap().cache.len());
    // dbg!(part1 / (num))
    Some((part1, 0))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p21.txt");
    #[test]
    fn day21_solve() {
        dbg!(super::solve(INPUT));
    }
}
