use std::fs::read_to_string;
use std::mem::swap;

type Board = Vec<Vec<char>>;

static FLOOR: char = '.';
static BUSY: char = '#';
static EMPTY: char = 'L';

fn main() {
    let input = read_to_string("input").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    part(input, 4, num_adjacent)
}

fn part2(input: &str) -> usize {
    part(input, 5, num_visible)
}

fn part<F>(input: &str, leave: usize, f: F) -> usize
where
    F: Fn(&Board, usize, usize) -> usize,
{
    let mut prev = load(input);
    let mut next = prev.clone();
    while step(&prev, &mut next, leave, &f) != 0 {
        swap(&mut prev, &mut next);
    }
    occupied(&next)
}

// Returns number of changes
fn step<F>(prev: &Board, next: &mut Board, leave_at: usize, weight_func: F) -> usize
where
    F: Fn(&Board, usize, usize) -> usize,
{
    let mut changes = 0;
    for (i, row) in prev.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if prev[i][j] == FLOOR {
                continue;
            }
            let na = weight_func(&prev, i, j);
            match na {
                0 => {
                    next[i][j] = BUSY;
                }
                x if x >= leave_at => {
                    next[i][j] = EMPTY;
                }
                _ => next[i][j] = prev[i][j],
            }
            if prev[i][j] != next[i][j] {
                changes += 1;
            }
        }
    }
    changes
}

fn num_adjacent(b: &Board, i: usize, j: usize) -> usize {
    let mut n = 0;
    let imin = if i == 0 { 0 } else { i - 1 }; // don't underflow unsigned int
    let jmin = if j == 0 { 0 } else { j - 1 };
    for ii in imin..i + 2 {
        if b.len() <= ii {
            continue; // out of bounds
        }
        for jj in jmin..j + 2 {
            if b[ii].len() <= jj {
                continue;
            }
            if i == ii && j == jj {
                continue;
            }
            if b[ii][jj] == BUSY {
                n += 1;
            }
        }
    }
    n
}

fn num_visible(b: &Board, i: usize, j: usize) -> usize {
    let (my, mx) = (b.len() as i32, b[0].len() as i32);
    look(b, i, j, my, mx, |y: i32, x: i32| (y - 1, x - 1))
        + look(b, i, j, my, mx, |y: i32, x: i32| (y - 1, x))
        + look(b, i, j, my, mx, |y: i32, x: i32| (y - 1, x + 1))
        + look(b, i, j, my, mx, |y: i32, x: i32| (y, x - 1))
        + look(b, i, j, my, mx, |y: i32, x: i32| (y, x + 1))
        + look(b, i, j, my, mx, |y: i32, x: i32| (y + 1, x - 1))
        + look(b, i, j, my, mx, |y: i32, x: i32| (y + 1, x))
        + look(b, i, j, my, mx, |y: i32, x: i32| (y + 1, x + 1))
}

// 0 if can't see a busy seat, 1 if can see one
fn look<F>(b: &Board, i: usize, j: usize, max_y: i32, max_x: i32, peer: F) -> usize
where
    F: Fn(i32, i32) -> (i32, i32),
{
    let (mut y, mut x) = peer(i as i32, j as i32);
    if !in_bounds(y, x, max_y, max_x) {
        return 0;
    }
    while b[y as usize][x as usize] == FLOOR {
        let peered = peer(y, x);
        y = peered.0;
        x = peered.1;
        if !in_bounds(y, x, max_y, max_x) {
            return 0;
        }
    }
    if b[y as usize][x as usize] == BUSY {
        1
    } else {
        0
    }
}

fn in_bounds(a: i32, b: i32, max_a: i32, max_b: i32) -> bool {
    (0..max_a).contains(&a) && (0..max_b).contains(&b)
}

fn occupied(b: &Board) -> usize {
    b.iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum()
}

fn load(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[allow(dead_code)]
fn print_board(b: &Board) {
    for row in b {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {

    static INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT), 26);
    }
}
