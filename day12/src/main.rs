use std::fs::read_to_string;

fn main() {
    println!("Part 1: {}", part1(&read_to_string("input").unwrap()));
    println!("Part 2: {}", part2(&read_to_string("input").unwrap()));
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    // angle, always a multiple of 90, 0 is up, 90 is East
    a: i32,
}

#[derive(Debug)]
struct Point {
    pos: Pos,
    w: Option<Pos>, // waypoint
}

impl Point {
    fn act(&mut self, a: &str) {
        let mut c = a.chars().next().unwrap();
        let mut val = a[1..].parse::<i32>().unwrap();
        if c == 'F' {
            if self.w.is_none() {
                // part1
                c = match self.pos.a {
                    0 => 'N',
                    90 => 'E',
                    180 => 'S',
                    270 => 'W',
                    _ => panic!("unknown direction: {}", self.pos.a),
                }
            } else {
                // part 2
                let wp = self.w.as_ref().unwrap();
                self.pos.x += wp.x * val;
                self.pos.y += wp.y * val;
                return;
            }
        }
        if c == 'L' {
            // convert left rotate into right
            val = ((val * -1) + 360) % 360;
            c = 'R';
        }
        // move the shop (part 1) or the waypoint (part 2)
        let mut target = if self.w.is_some() {
            self.w.as_mut().unwrap()
        } else {
            &mut self.pos
        };
        match c {
            'N' => target.y -= val,
            'S' => target.y += val,
            'E' => target.x += val,
            'W' => target.x -= val,
            'R' => {
                if self.w.is_none() {
                    // part 1
                    self.pos.a += val;
                    self.pos.a %= 360;
                } else {
                    // part 2
                    let wp = self.w.as_mut().unwrap();
                    // TODO: extract a 'translate' function
                    match val {
                        90 => {
                            // a,b -> -b, a
                            let temp = wp.x;
                            wp.x = -1 * wp.y;
                            wp.y = temp;
                        }
                        180 => {
                            // a,b -> -a,-b
                            wp.x *= -1;
                            wp.y *= -1;
                        }
                        270 => {
                            // a,b -> b,-a
                            let temp = wp.x;
                            wp.x = wp.y;
                            wp.y = -1 * temp;
                        }
                        _ => panic!("wrong angle {}", val),
                    }
                }
            }
            _ => panic!("Unknown action: {}", c),
        }
    }

    fn dist(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

fn part1(input: &str) -> i32 {
    let mut t = Point {
        pos: Pos { x: 0, y: 0, a: 90 },
        w: None,
    };
    input.lines().for_each(|x| t.act(x));
    t.dist()
}

fn part2(input: &str) -> i32 {
    let mut t = Point {
        pos: Pos { x: 0, y: 0, a: 90 },
        w: Some(Pos { x: 10, y: -1, a: 0 }),
    };
    input.lines().for_each(|x| t.act(x));
    t.dist()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = r"F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT), 286);
    }
}
