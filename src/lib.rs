pub mod p1;
pub mod p10;
pub mod p11;
pub mod p12;
pub mod p13;
pub mod p14;
pub mod p15;
pub mod p16;
pub mod p17;
pub mod p18;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
}

impl Direction {
    fn all_directions() -> [Self; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
    fn orthogonal(origin: (usize, usize)) -> [(usize, usize); 4] {
        [
            Direction::North.get_square(origin),
            Direction::East.get_square(origin),
            Direction::South.get_square(origin),
            Direction::West.get_square(origin),
        ]
    }
    fn get_square(self, origin: (usize, usize)) -> (usize, usize) {
        let (row, col) = origin;
        match self {
            Direction::North => (row.wrapping_sub(1), col),
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => (row, col.wrapping_sub(1)),
        }
    }
    fn turn(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

pub fn simple_bench<O>(input: &str, function: fn(&str) -> O) {
    benchmarking::warm_up();

    let bench_result = benchmarking::bench_function(|measurer| {
        measurer.measure(|| function(input));
    })
    .unwrap();
    assert!(bench_result.times() > 0);
    eprintln!("Average duration: {:?}", bench_result.elapsed());
}
