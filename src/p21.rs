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
            lookup: lookup,
            location: 10,
            parent,
        }
    }
    fn next_number(&mut self, goal: u8) -> u64 {
        let paths = self.iterate_paths(self.location, goal);
        let mut best_cost = u64::MAX;
        for path in paths {
            let p = self.parent.walk(path);
            if p.absolute_path.len() < best_cost {
                best_cost = p.absolute_path.len();
            }
        }
        self.location = goal;
        best_cost
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

#[derive(Clone, Default)]
struct CacheState {
    data: Vec<PadVal>,
    absolute_path: CompressedPath,
}

impl CacheState {
    fn from_vec(vec: Vec<PadVal>, absolute_path: CompressedPath) -> Self {
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
        for (k, v) in self.absolute_path.data.iter() {
            write!(f, "{:?} : {:?}", *k, *v)?;
        }
        writeln!(f)
    }
}

#[derive(Default, Clone)]
struct CompressedPath {
    data: HashMap<Vec<PadVal>, u64>,
}

impl CompressedPath {
    fn len(&self) -> u64 {
        let mut sum = 0;
        for (k, v) in self.data.iter() {
            sum += k.len() as u64 * *v;
        }
        sum
    }
    fn extend(&mut self, other: Self) {
        for (k, v) in other.data {
            *self.data.entry(k).or_default() += v;
        }
    }
}

impl From<Vec<PadVal>> for CompressedPath {
    fn from(value: Vec<PadVal>) -> Self {
        // Check that is it just one path
        assert_eq!(value.last(), Some(&PadVal::A));
        let head = &value[0..value.len() - 1];
        assert!(head.iter().find(|&&x| x == PadVal::A).is_none());
        // All good, insert it
        let mut data = HashMap::new();
        data.insert(value, 1);
        Self { data }
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
            // Check bounds by walking the whole path
            let mut next = st;
            for s in perm.iter() {
                next = s.step(next);
                if next == Self::hole() {
                    continue 'outer;
                }
            }
            perm.push(PadVal::A); // Always ends with A
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
            lookup,
            cache: HashMap::new(),
            location: PadVal::A,
            parent,
        }
    }
    fn walk(&mut self, vec: Vec<PadVal>) -> CacheState {
        let mut state = self.location;
        let mut absolute = CompressedPath::default();
        for val in vec.iter().copied() {
            let path = self.get_path(state, val);
            absolute.extend(path.absolute_path.clone());
            state = val;
        }
        assert_eq!(state, PadVal::A);
        CacheState::from_vec(vec, absolute)
    }
    fn get_path(&mut self, st: PadVal, end: PadVal) -> &CacheState {
        let key = CacheKey::new(st, end);
        if st == end {
            return self.cache.entry(key).or_insert_with(|| {
                CacheState::from_vec(vec![PadVal::A], CompressedPath::from(vec![PadVal::A]))
            });
        }
        if self.cache.contains_key(&key) {
            return self.cache.get(&key).unwrap();
        }
        let all_paths = self.iterate_paths(st, end);
        let mut best_path = CacheState::default();
        let mut min_cost = u64::MAX;
        for path in all_paths {
            if let Some(parent) = self.parent.as_mut() {
                let check = parent.walk(path);
                if check.absolute_path.len() < min_cost {
                    min_cost = check.absolute_path.len();
                    best_path = check;
                }
            } else {
                if (path.len() as u64) < min_cost {
                    min_cost = path.len() as u64;
                    best_path = CacheState::from_vec(path.clone(), CompressedPath::from(path));
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

fn depth_solve(input: &str, depth: usize) -> Option<u64> {
    let mut numpad = build_numpad(depth);
    let mut ans = 0;
    for line in input.lines() {
        let seq = line.chars().map(|x| x.to_digit(16).unwrap() as u8);
        let numeric: String = line.chars().filter(|x| x.is_ascii_digit()).collect();
        let numeric_num = u64::from_str_radix(&numeric, 10).ok()?;
        let mut cost = 0;
        for dest in seq {
            cost += numpad.next_number(dest);
        }
        ans += cost * numeric_num;
        numpad.reset();
    }
    Some(ans)
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    Some((depth_solve(input, 2)?, depth_solve(input, 25)?))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p21.txt");
    #[test]
    fn day21_solve() {
        dbg!(super::solve(INPUT));
    }
}
