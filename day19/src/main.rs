use std::collections::HashMap;
use std::fmt;
use std::fs;

fn main() {
    println!("Part 1: {}", part1(&fs::read_to_string("input").unwrap()));
}

type Rules = HashMap<usize, Rule>;

fn part1(input: &str) -> usize {
    let mut iterator = input.lines();
    let rules = load_rules(&mut iterator);
    let r0 = rules.get(&0).unwrap();

    let mut result = 0;
    for message in iterator {
        let mchars: Vec<char> = message.chars().collect();
        let (is_match, match_len) = r0.matches(&mchars, &rules);
        if is_match && match_len == mchars.len() {
            result += 1;
        }
    }
    result
}

fn load_rules<'a>(iterator: &mut impl Iterator<Item = &'a str>) -> Rules {
    let mut out = HashMap::new();
    for line in iterator {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(":");
        let rule_num: usize = parts.next().unwrap().parse().unwrap();
        let rule = parts.next().unwrap().trim();

        // single char case: 4: "a"
        if rule.starts_with('"') {
            out.insert(rule_num, Rule::Char(rule.chars().nth(1).unwrap()));
            continue;
        }

        // sub-rule case: 1: 2 3 | 3 2
        let mut left = Vec::new();
        let mut right = None;
        for c in rule.split(' ') {
            match c {
                "|" => right = Some(Vec::new()),
                _ => {
                    let sub_rule: usize = c.parse().unwrap();
                    if right.is_none() {
                        left.push(sub_rule);
                    } else {
                        right.as_mut().unwrap().push(sub_rule);
                    }
                }
            }
        }
        out.insert(rule_num, Rule::Sub { left, right });
    }
    out
}

#[derive(Debug)]
enum Rule {
    Char(char),
    Sub {
        left: Vec<usize>,
        right: Option<Vec<usize>>,
    },
}

impl Rule {
    fn matches(&self, m: &[char], rules: &Rules) -> (bool, usize) {
        match self {
            Self::Char(c) => (*c == m[0], 1),
            Self::Sub { left, right } => {
                let (l_match, l_moves) = sub_matches(&left, m, rules);
                if l_match {
                    return (true, l_moves);
                }
                if right.is_some() {
                    let (r_match, r_moves) = sub_matches(right.as_ref().unwrap(), m, rules);
                    if r_match {
                        return (true, r_moves);
                    }
                }
                (false, 0)
            }
        }
    }
}

fn sub_matches(sub_rules: &Vec<usize>, m: &[char], rules: &Rules) -> (bool, usize) {
    let mut pos = 0;
    for rnum in sub_rules {
        let rule = rules.get(&rnum).unwrap();
        let (is_match, moves) = rule.matches(&m[pos..], rules);
        if !is_match {
            return (false, 0);
        }
        pos += moves;
    }
    (true, pos)
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(c) => write!(f, "{}", c),
            Self::Sub { left, right } => {
                let mut result = write!(f, "{:?}", left);
                if right.is_some() {
                    result = write!(f, " | {:?}", right.as_ref().unwrap());
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {

    static INPUT_SMALL: &str = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

aab
aba
"#;

    static INPUT_BIG: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT_SMALL), 2);
        assert_eq!(super::part1(INPUT_BIG), 2);
    }
}
