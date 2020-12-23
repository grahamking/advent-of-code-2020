use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let p1 = part1(25, &read_to_string("input").unwrap());
    println!("part1: {}", p1);
    println!("part2: {}", part2(p1, &read_to_string("input").unwrap()));
}

fn part1(plen: usize, input: &str) -> usize {
    let nums: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut end = plen;
    for (start, n) in nums.iter().skip(plen).enumerate() {
        let valid = sums(&nums[start..end]);
        if !valid.contains(n) {
            return *n;
        }
        end += 1;
    }
    panic!("not found");
}

fn sums(nums: &[usize]) -> HashSet<usize> {
    let mut valid = HashSet::<usize>::new();
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            valid.insert(x + y);
        }
    }
    valid
}

fn part2(p1: usize, input: &str) -> usize {
    let nums: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut components = Vec::new();
    let mut sum = 0;
    let mut pos = 0;
    while pos < nums.len() {
        let x = nums[pos];
        components.push(x);
        sum += x;
        while sum > p1 {
            components.remove(0);
            sum = components.iter().sum();
        }
        if sum == p1 {
            return components.iter().min().unwrap() + components.iter().max().unwrap();
        }
        pos += 1;
    }
    panic!("not found");
}

#[cfg(test)]
mod tests {

    static INPUT: &str = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(5, &INPUT), 127);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(127, &INPUT), 62);
    }
}
