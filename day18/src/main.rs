use std::fs;

fn main() {
    println!("Part 1: {}", part1(&fs::read_to_string("input").unwrap()));
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let exp: Vec<char> = line.chars().filter(|c| *c != ' ').collect();
        let (result, _) = calc(&exp);
        total += result;
    }
    total
}

fn calc(exp: &[char]) -> (usize, usize) {
    let mut nums: Vec<usize> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let mut pos = 0;
    let mut val;
    while pos < exp.len() {
        val = exp[pos];
        pos += 1;
        match val {
            '+' | '*' => ops.push(val),
            '(' => {
                let (result, moves) = calc(&exp[pos..]);
                nums.push(result);
                pos += moves;
            }
            ')' => break,
            x => nums.push(x.to_digit(10).unwrap() as usize),
        }
        if nums.len() == 2 {
            let left = match ops.pop().unwrap() {
                '+' => nums.pop().unwrap() + nums.pop().unwrap(),
                '*' => nums.pop().unwrap() * nums.pop().unwrap(),
                _ => panic!("invalid operator"),
            };
            nums.push(left);
        }
    }
    (nums.pop().unwrap(), pos)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(super::part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(super::part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            super::part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            super::part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
