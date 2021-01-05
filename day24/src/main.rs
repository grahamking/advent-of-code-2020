use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    println!("Part 1: {}", part1(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut flipped = HashSet::new();
    for line in input.lines() {
        let pos = StepIter::new(line).fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));
        if !flipped.remove(&pos) {
            flipped.insert(pos);
        }
    }
    Ok(flipped.len())
}

type Move = (i8, i8);

// This is the clever part, it maps a hex layout into a 2D grid. I did not invent it.
static MOVES: [Move; 6] = [
    (1, 0),  // e
    (0, 1),  // se
    (-1, 1), // sw
    (-1, 0), // w
    (0, -1), // nw
    (1, -1), // ne
];

struct StepIter<'a> {
    chars: Box<dyn Iterator<Item = char> + 'a>,
}

impl<'a> Iterator for StepIter<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.chars.next() {
            Some(c) => c,
            None => return None,
        };
        let idx = match c {
            'e' => 0,
            's' => match self.chars.next().unwrap() {
                'e' => 1,
                'w' => 2,
                x => panic!("unknown move {}{}", c, x),
            },
            'w' => 3,
            'n' => match self.chars.next().unwrap() {
                'w' => 4,
                'e' => 5,
                x => panic!("unknown move {}{}", c, x),
            },
            _ => panic!("unknown move {}", c),
        };
        Some(MOVES[idx])
    }
}

impl<'a> StepIter<'a> {
    fn new(line: &str) -> StepIter {
        StepIter {
            chars: Box::new(line.chars()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    static INPUT: &str = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(super::part1(INPUT)?, 10);
        Ok(())
    }
}
