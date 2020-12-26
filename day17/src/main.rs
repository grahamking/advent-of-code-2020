// Advent of Code 2020: Day 17

use std::collections::HashSet;

static INPUT: &str = r"
.##...#.
.#.###..
..##.#.#
##...#.#
#..#...#
#..###..
.##.####
..#####.";

fn main() {
    println!("Part 1: {}", part(INPUT, 3));
    println!("Part 2: {}", part(INPUT, 4));
}

fn part(input: &str, dim: i32) -> usize {
    let mut b = Board::new(input, dim);
    for _ in 0..6 {
        b.advance();
    }
    b.num_active()
}

#[derive(Debug)]
struct Board {
    dim: i32,
    active: HashSet<[i32; 4]>,
    next: HashSet<[i32; 4]>,
}

impl Board {
    fn new(input: &str, dim: i32) -> Self {
        let data: Vec<&str> = if input.lines().next().unwrap().is_empty() {
            input.lines().skip(1).collect()
        } else {
            input.lines().collect()
        };
        let mut active = HashSet::new();
        let z: i32 = 0;
        let w: i32 = 0;
        for (y, l) in data.iter().enumerate() {
            for (x, val) in l.chars().enumerate() {
                if val == '#' {
                    active.insert([x as i32, y as i32, z, w]);
                }
            }
        }
        Board {
            active,
            dim,
            next: HashSet::new(),
        }
    }

    fn num_active(&self) -> usize {
        self.active.len()
    }

    fn advance(&mut self) {
        self.next = self.active.clone();
        self.consider_active();
        self.consider_inactive();
        self.active = self.next.clone();
    }

    // If a cube is active and exactly 2 or 3 of its neighbors are
    // also active, the cube remains active. Otherwise, the cube
    // becomes inactive.
    fn consider_active(&mut self) {
        for pos in self.active.iter() {
            let num_n = self.count_neighbors(pos[0], pos[1], pos[2], pos[3]);
            if num_n != 2 && num_n != 3 {
                //println!("becomes active: {:?}", pos);
                self.next.remove(pos);
            }
        }
    }

    // If a cube is inactive but exactly 3 of its neighbors are
    // active, the cube becomes active. Otherwise, the cube remains inactive.
    fn consider_inactive(&mut self) {
        // Consider a cube that wraps our max active cube, and check all points
        // within that.
        let (min, max) = self.bounds();
        let min = min - 1;
        let max = max + 1;
        // TODO: make recursive by dim, fewer loops
        for x in min..=max {
            for y in min..=max {
                for z in min..=max {
                    if self.dim == 3 {
                        // part 1
                        if self.active.contains(&[x, y, z, 0]) {
                            continue;
                        }
                        let num_n = self.count_neighbors(x, y, z, 0);
                        if num_n == 3 {
                            self.next.insert([x, y, z, 0]);
                        }
                    } else {
                        // part 2
                        for w in min..=max {
                            if self.active.contains(&[x, y, z, w]) {
                                continue;
                            }
                            let num_n = self.count_neighbors(x, y, z, w);
                            if num_n == 3 {
                                self.next.insert([x, y, z, w]);
                            }
                        } // for w
                    }
                } // for z
            } // for y
        } // for x
    }

    fn count_neighbors(&self, center_x: i32, center_y: i32, center_z: i32, center_w: i32) -> usize {
        let mut count = 0;
        for x in -1..=1 {
            let cx = center_x + x; // candidate X
            for y in -1..=1 {
                let cy = center_y + y;
                for z in -1..=1 {
                    let cz = center_z + z;

                    if self.dim == 3 {
                        // part 1
                        if x == 0 && y == 0 && z == 0 {
                            continue;
                        }
                        if self.active.contains(&[cx, cy, cz, 0]) {
                            count += 1;
                        }
                    } else {
                        // part 2
                        for w in -1..=1 {
                            let cw = center_w + w;
                            if x == 0 && y == 0 && z == 0 && w == 0 {
                                continue;
                            }
                            if self.active.contains(&[cx, cy, cz, cw]) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }

    // (min, max) of any of x,y,z
    fn bounds(&self) -> (i32, i32) {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for key in self.active.iter() {
            let (x, y, z, w) = (key[0], key[1], key[2], key[3]);
            //println!("{}, {}, {}", x, y, z);
            if x < min {
                min = x;
            }
            if y < min {
                min = y;
            }
            if z < min {
                min = z;
            }
            if w < min {
                min = w;
            }

            if x > max {
                max = x;
            }
            if y > max {
                max = y;
            }
            if z > max {
                max = z;
            }
            if w > max {
                max = w
            }
        }
        (min, max)
    }

    fn max_z(&self) -> i32 {
        let mut max = i32::MIN;
        for key in self.active.iter() {
            let z = key[2];
            if z > max {
                max = z;
            }
        }
        max
    }

    #[allow(dead_code)]
    fn show(&self) {
        // 3d only
        let (min, max) = self.bounds();
        for z in min..=self.max_z() {
            println!("\nz={}", z);
            for y in min..=max {
                for x in min..=max {
                    if self.active.contains(&[x, y, z, 0]) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r"
.#.
..#
###";

    #[test]
    fn test_count_neighbors() {
        let b = super::Board::new(TEST_INPUT, 3);
        assert_eq!(b.count_neighbors(0, 0, 0, 0), 1);
        assert_eq!(b.count_neighbors(2, 1, 0, 0), 3);
        assert_eq!(b.bounds(), (0, 2));
    }

    #[test]
    fn test_bounds() {
        let mut b = super::Board::new(TEST_INPUT, 3);
        b.advance();
        b.show();
        assert_eq!(b.bounds(), (-1, 3));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part(TEST_INPUT, 3), 112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part(TEST_INPUT, 4), 848);
    }
}
