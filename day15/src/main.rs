use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1(&vec![14, 8, 16, 0, 1, 17], 2020));
    println!("Part 2: {}", part1(&vec![14, 8, 16, 0, 1, 17], 30000000));
}

fn part1(input: &[usize], r: usize) -> usize {
    let mut m = HashMap::new();
    let mut prev;
    let l = input.len();
    for (i, val) in input.iter().enumerate() {
        m.insert(*val, i + 1); // i+1 is 1-based turn it was last spoken
    }
    prev = input[l - 1];
    for turn in l..r {
        let next = match m.get(&prev) {
            None => 0,
            Some(last_spoken) => turn - last_spoken,
        };
        m.insert(prev, turn);
        prev = next;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![0, 3, 6], 4), 0);
        assert_eq!(part1(&vec![0, 3, 6], 5), 3);
        assert_eq!(part1(&vec![0, 3, 6], 6), 3);
        assert_eq!(part1(&vec![0, 3, 6], 2020), 436);
    }
}
