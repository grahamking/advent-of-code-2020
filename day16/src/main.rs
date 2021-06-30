use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::sync::mpsc;
use std::thread;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loader(tx));
    println!("Part 1: {}", part1(rx.into_iter()));
}

// Unnecessary, but I wanted a thread and a channel for my own entertainment.
// Advent of code is for fun!
fn loader(c: mpsc::Sender<String>) {
    let f = File::open("input").unwrap();
    for line in BufReader::new(f).lines() {
        c.send(line.unwrap()).unwrap();
    }
}

// can't use IntoIterator because I'm passing str.lines()
fn part1(mut iter: impl Iterator<Item = String>) -> usize {
    // load rule set
    let mut rs = RuleSet(Vec::new());
    loop {
        let l = iter.next().unwrap();
        if l.is_empty() {
            // end of rules
            break;
        }
        rs.add(l.as_str().into());
    }

    // skip "your ticket", your ticket, blank, and "nearby tickets"
    let mut iter = iter.skip(4);

    // match tickets against rules
    let mut scan_err_rate = 0;
    while let Some(l) = iter.next() {
        let t: Ticket = l.as_str().into();
        scan_err_rate += rs.misses(&t).iter().sum::<usize>();
    }

    scan_err_rate
}

#[derive(Debug)]
struct RuleSet(Vec<Rule>);
impl RuleSet {
    // add a rule
    fn add(&mut self, r: Rule) {
        self.0.push(r);
    }

    // all the numbers in this ticket that match none of the rules
    fn misses(&self, ticket: &Ticket) -> Vec<usize> {
        let mut out = Vec::new();
        for num in &ticket.0 {
            if self.0.iter().any(|rule| rule.matches(*num)) {
                continue;
            }
            out.push(*num);
        }
        out
    }
}

#[derive(Debug)]
struct Rule {
    name: String,
    r1: RangeInclusive<usize>,
    r2: RangeInclusive<usize>,
}

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"(.+): (\d+\-\d+) or (\d+\-\d+)").unwrap();
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let cap = RULE_RE.captures(s).unwrap();
        let r1p: Vec<usize> = cap
            .get(2)
            .unwrap()
            .as_str()
            .split('-')
            .filter_map(|x| x.parse().ok())
            .collect();
        let r2p: Vec<usize> = cap
            .get(3)
            .unwrap()
            .as_str()
            .split('-')
            .filter_map(|x| x.parse().ok())
            .collect();
        Rule {
            name: cap.get(1).unwrap().as_str().into(),
            r1: r1p[0]..=r1p[1],
            r2: r2p[0]..=r2p[1],
        }
    }
}

impl Rule {
    fn matches(&self, n: usize) -> bool {
        self.r1.contains(&n) || self.r2.contains(&n)
    }
}

#[derive(Debug)]
struct Ticket(Vec<usize>);
impl From<&str> for Ticket {
    fn from(s: &str) -> Self {
        let v = s.split(',').filter_map(|x| x.parse().ok()).collect();
        Ticket(v)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT.lines().map(String::from)), 71);
    }
}
