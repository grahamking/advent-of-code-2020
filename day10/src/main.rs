use std::collections::HashMap;
use std::fs;

fn main() {
    println!("part 1: {}", part1(&fs::read_to_string("input").unwrap()));
    println!("part 2: {}", part2(&fs::read_to_string("input").unwrap()));
}

fn part1(input: &str) -> usize {
    let mut ones = 0;
    let mut threes = 0;
    let mut prev = 0;
    for j in load(input) {
        match j {
            _ if j == prev + 1 => ones += 1,
            _ if j == prev + 2 => {}
            _ if j == prev + 3 => threes += 1,
            _ => panic!("{} is not {} + 1|2|3", j, prev),
        }
        prev = j;
    }
    ones * threes
}

fn part2(input: &str) -> usize {
    let mut paths = HashMap::new();
    paths.insert(0, 1);
    for j in load(input) {
        paths.insert(j, 0);
        for step in 1..4 {
            if paths.contains_key(&(j - step)) {
                let v = *paths.get(&(j - step)).unwrap();
                *paths.get_mut(&j).unwrap() += v;
            }
        }
    }
    let last_key = paths.keys().max().unwrap();
    *paths.get(last_key).unwrap()
}

fn load(input: &str) -> Vec<i32> {
    let mut adaptors: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    adaptors.sort();
    adaptors.push(adaptors[adaptors.len() - 1] + 3); // device joltage
    return adaptors;
}

#[cfg(test)]
mod tests {

    static INPUT_SMALL: &str = r"16
10
15
5
1
11
7
19
6
12
4";

    static INPUT_MED: &str = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT_SMALL), 7 * 5);
        assert_eq!(super::part1(INPUT_MED), 22 * 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT_SMALL), 8);
        assert_eq!(super::part2(INPUT_MED), 19208);
    }
}
