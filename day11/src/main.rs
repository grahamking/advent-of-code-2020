use std::fs::read_to_string;
use std::mem::swap;

type Board = Vec<Vec<char>>;

fn main() {
    let input = read_to_string("input").unwrap();
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let mut prev = load(input);
    let mut next = prev.clone();
    while step(&prev, &mut next) != 0 {
        swap(&mut prev, &mut next);
    }
    occupied(&next)
}

// Returns number of changes
fn step(prev: &Board, next: &mut Board) -> usize {
    let mut changes = 0;
    for (i, row) in prev.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if prev[i][j] == '.' {
                continue;
            }
            let na = num_adjacent(&prev, i, j);
            match na {
                0 => {
                    next[i][j] = '#';
                }
                x if x >= 4 => {
                    next[i][j] = 'L';
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
            if b[ii][jj] == '#' {
                n += 1;
            }
        }
    }
    n
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
}
