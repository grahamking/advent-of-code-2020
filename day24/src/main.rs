use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    println!("Part 1: {}", part1(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut moves = HashMap::new();
    moves.insert("e", (1, 0));
    moves.insert("se", (0, 1));
    moves.insert("sw", (-1, 1));
    moves.insert("w", (-1, 0));
    moves.insert("nw", (0, -1));
    moves.insert("ne", (1, -1));

    let mut flipped = HashSet::new();
    for line in input.lines() {
        let mut pos = (0, 0);
        for m in StepIter::new(line) {
            let (x, y) = moves.get(m).ok_or_else(|| format!("missing move {}", m))?;
            pos.0 += x;
            pos.1 += y;
        }
        if flipped.contains(&pos) {
            flipped.remove(&pos);
        } else {
            flipped.insert(pos);
        }
    }
    Ok(flipped.len())
}

struct StepIter<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Iterator for StepIter<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.chars.next() {
            Some(c) => c,
            None => return None,
        };
        let out = match c {
            'e' => "e",
            'w' => "w",
            's' => match self.chars.next().unwrap() {
                'e' => "se",
                'w' => "sw",
                x => panic!("unknown move {}{}", c, x),
            },
            'n' => match self.chars.next().unwrap() {
                'e' => "ne",
                'w' => "nw",
                x => panic!("unknown move {}{}", c, x),
            },
            _ => panic!("unknown move {}", c),
        };
        Some(out)
    }
}

impl<'a> StepIter<'a> {
    fn new(line: &str) -> StepIter {
        StepIter {
            chars: line.chars(),
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
