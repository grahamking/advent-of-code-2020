use std::collections::HashSet;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = read_to_string("input").unwrap();
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let tiles: Vec<Tile> = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(Tile::from_str)
        .filter_map(|x| x.ok())
        .collect();
    let grid_side = (tiles.len() as f32).sqrt();
    println!("Grid: {}x{}", grid_side, grid_side);

    let corners = top_left(&tiles);
    corners.iter().map(|x| x.num).product()
}

// Find the top left tile - it should match on the right and the bottom only
// This actually finds all four corners. I don't know why it works!
// It's late, I'll understand it tomorrow.
fn top_left(tiles: &[Tile]) -> Vec<Tile> {
    let mut top_left = Vec::new();
    for original in tiles.iter() {
        'options: for t in original.options() {
            let mut is_above = false;
            let mut is_left = false;
            for orig_u in tiles.iter() {
                if t.num == orig_u.num {
                    continue;
                }
                for u in orig_u.options() {
                    if t.left(&u) {
                        is_left = true;
                    }
                    if t.above(&u) {
                        is_above = true;
                    }
                    if t.right(&u) || t.below(&u) {
                        // t in this orientation cannot be top-left
                        continue 'options;
                    }
                }
            }
            if is_above && is_left {
                top_left.push(t.clone());
                break;
            }
        }
    }
    top_left
}

#[derive(Debug, Clone)]
struct Tile {
    num: usize,
    edges: HashSet<usize>,
}

fn _nice(s: &HashSet<usize>) -> Vec<usize> {
    let mut v: Vec<usize> = s.iter().map(|x| *x).collect();
    v.sort();
    v
}

impl Tile {
    fn options(&self) -> impl Iterator<Item = Tile> {
        let mut out = Vec::new();
        out.push(self.clone());

        let r90 = self.rotated();
        let r180 = r90.rotated();
        let r270 = r180.rotated();
        out.push(r90);
        out.push(r180);
        out.push(r270);

        let flip = self.flipped();
        out.push(flip.clone());

        let r90 = flip.rotated();
        let r180 = r90.rotated();
        let r270 = r180.rotated();
        out.push(r90);
        out.push(r180);
        out.push(r270);

        out.into_iter()
    }

    // This tile rotated 90 degrees clockwise
    fn rotated(&self) -> Tile {
        let mut edges = HashSet::new();
        for e in self.edges.iter() {
            // feels awkward, maybe a better way?
            let next = match e {
                0..=10 => e * 10,
                91..=100 => (e - 91) * 10 + 1,
                x if x % 10 == 0 => 100 - ((x - 10) / 10),
                x if (x + 9) % 10 == 0 => 10 - ((x - 1) / 10),
                _ => panic!("not an edge: {}", e),
            };
            edges.insert(next);
        }
        Tile {
            num: self.num,
            edges,
        }
    }

    // This tile flipped along it's up-down axis. Left becomes right.
    fn flipped(&self) -> Tile {
        let mut edges = HashSet::new();
        for e in self.edges.iter() {
            // feels awkward, maybe a better way?
            let next = match e {
                0..=10 => 10 - e + 1,
                91..=100 => 100 - (e - 90) + 1,
                x if x % 10 == 0 => x - 9,
                x if (x + 9) % 10 == 0 => x + 9,
                _ => panic!("not an edge: {}", e),
            };
            edges.insert(next);
        }
        Tile {
            num: self.num,
            edges,
        }
    }

    // Is self above u
    fn above(&self, u: &Tile) -> bool {
        let t_bottom = self.bottom_side();
        let u_top = u.top_side();
        if u_top.len() != t_bottom.len() {
            return false;
        }
        for e in t_bottom.iter() {
            if !u_top.contains(&(e - 90)) {
                return false;
            }
        }
        true
    }

    // Is self below u
    fn below(&self, u: &Tile) -> bool {
        u.above(self)
    }

    // Is self left of u
    fn left(&self, u: &Tile) -> bool {
        let t_right = self.right_side();
        let u_left = u.left_side();
        if t_right.len() != u_left.len() {
            return false;
        }
        for e in t_right.iter() {
            if !u_left.contains(&(e - 9)) {
                return false;
            }
        }
        true
    }

    // Is self right of u
    fn right(&self, u: &Tile) -> bool {
        u.left(self)
    }

    fn top_side(&self) -> Vec<usize> {
        self.edges
            .iter()
            .map(|x| *x)
            .filter(|x| (1..=10).contains(x))
            .collect()
    }
    fn bottom_side(&self) -> Vec<usize> {
        self.edges
            .iter()
            .map(|x| *x)
            .filter(|x| (91..=100).contains(x))
            .collect()
    }
    fn right_side(&self) -> Vec<usize> {
        self.edges
            .iter()
            .map(|x| *x)
            .filter(|x| x % 10 == 0)
            .collect()
    }
    fn left_side(&self) -> Vec<usize> {
        self.edges
            .iter()
            .map(|x| *x)
            .filter(|x| (x + 9) % 10 == 0)
            .collect()
    }
}

impl FromStr for Tile {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let num = iter
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();

        let mut edges = HashSet::new();
        let mut pos = 1;
        for line in iter {
            for char in line.chars() {
                if char == '#' && is_edge(pos) {
                    edges.insert(pos);
                }
                pos += 1;
            }
        }

        Ok(Tile { num, edges })
    }
}

// it's late
static EDGES: [usize; 36] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 21, 31, 41, 51, 61, 71, 81, 20, 30, 40, 50, 60, 70, 80, 90,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
];

fn is_edge(x: usize) -> bool {
    EDGES.contains(&x)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("small-input").unwrap();
        assert_eq!(super::part1(&input), 20899048083289);
    }
}
