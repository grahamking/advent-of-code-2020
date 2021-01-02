use std::cmp::Ordering;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    println!("Part 1: {}", part1(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut p_iter = input.split("\n\n");

    let mut p1: VecDeque<usize> = p_iter
        .next()
        .unwrap()
        .lines()
        .skip(1) // skip "Player 1:"
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut p2: VecDeque<usize> = p_iter
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let win = loop {
        let c1 = p1.pop_front().ok_or("p1 no cards")?;
        let c2 = p2.pop_front().ok_or("p2 no cards")?;
        match c1.cmp(&c2) {
            Ordering::Greater => {
                // player 1 wins the round
                p1.push_back(c1);
                p1.push_back(c2);
            }
            Ordering::Less => {
                // player 2 wins the round
                p2.push_back(c2);
                p2.push_back(c1);
            }
            Ordering::Equal => {
                // never happens
                panic!("cards are equal, unspecified behavior");
            }
        }
        if p1.is_empty() {
            break p2;
        } else if p2.is_empty() {
            break p1;
        }
    };

    Ok(win
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i + 1) * x))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    static INPUT: &str = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(super::part1(INPUT)?, 306);
        Ok(())
    }
}
