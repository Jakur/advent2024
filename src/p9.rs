use std::usize;

const EMPTY: usize = usize::MAX;

fn get_number(val: u8) -> u8 {
    return val - b'0';
}

#[derive(Debug, Clone, Copy)]
struct Block {
    id: usize,
    location: usize,
    size: u8,
}

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let mut fs = Vec::new();
    let mut space = Vec::new();
    let mut used = Vec::new();
    for (id, val) in input
        .lines()
        .nth(0)
        .unwrap()
        .as_bytes()
        .chunks(2)
        .enumerate()
    {
        let file_len = get_number(val[0]);
        used.push(Block {
            id,
            location: fs.len(),
            size: file_len,
        });
        for _ in 0..file_len {
            fs.push(id);
        }
        if let Some(second) = val.get(1) {
            let free_space = get_number(*second);
            space.push(Block {
                id: EMPTY,
                location: fs.len(),
                size: free_space,
            });
            for _ in 0..free_space {
                fs.push(EMPTY);
            }
        }
    }
    let mut fs2 = fs.clone();

    // Part 1
    let mut backward_idx = fs.len() - 1;
    let mut forward_idx = 0;
    while forward_idx < backward_idx {
        let val = fs[forward_idx];
        if val == EMPTY {
            while fs[backward_idx] == EMPTY {
                backward_idx -= 1;
            }
            let val = fs[backward_idx];
            fs[backward_idx] = EMPTY;
            fs[forward_idx] = val;
        } else {
            forward_idx += 1;
        }
    }
    // Part 2
    for block in used.into_iter().rev() {
        let space_block = space
            .iter_mut()
            .find(|x| x.size >= block.size && x.location < block.location);
        if let Some(space_block) = space_block {
            for i in 0..block.size {
                fs2[i as usize + space_block.location] = block.id;
                fs2[i as usize + block.location] = EMPTY;
            }
            // Just leave the 0 capacity empty blocks alone, to be skipped over
            let remaining = space_block.size - block.size;
            space_block.size = remaining;
            space_block.location += block.size as usize;
        }
    }
    let part1 = score(&fs);
    let part2 = score(&fs2);
    Some((part1, part2))
}

fn score(fs: &[usize]) -> usize {
    fs.iter()
        .enumerate()
        .filter(|(_, val)| **val != EMPTY)
        .map(|(i, val)| i * val)
        .sum()
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p9.txt");
    #[test]
    fn day9_solve() {
        dbg!(super::solve(INPUT));
    }
}
