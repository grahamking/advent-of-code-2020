use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let f = File::open("input")?;
    println!("Part 1: {}", part1(&f));
    Ok(())
}

fn part1(input: impl Read) -> u64 {
    // 2^36 is 68719476736, the input is only 574 lines long. Most values will
    // be empty, so fake it with a HashMap
    let mut mem = HashMap::new();

    let mut mask: Option<Mask> = None;
    for line in BufReader::new(input).lines().filter_map(|l| l.ok()) {
        if line.starts_with("mask") {
            mask = Some(line.as_str().into());
            continue;
        }
        let op: Op = line.as_str().into();
        let v = mask.as_ref().unwrap().apply(op.val);
        mem.insert(op.pos, v);
    }

    mem.values().sum()
}

#[derive(Debug)]
struct Op {
    pos: usize,
    val: u64,
}

impl From<&str> for Op {
    fn from(s: &str) -> Op {
        lazy_static! {
            static ref OP_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }
        let cap = OP_RE.captures(s).unwrap();
        Op {
            pos: cap.get(1).unwrap().as_str().parse().unwrap_or(0),
            val: cap.get(2).unwrap().as_str().parse().unwrap_or(0),
        }
    }
}

#[derive(Debug)]
struct Mask {
    zeros: u64,
    ones: u64,
}
impl From<&str> for Mask {
    fn from(s: &str) -> Mask {
        let v = String::from(s.split("=").nth(1).unwrap().trim());
        let ones: u64 = u64::from_str_radix(&v.replace("X", "0"), 2).unwrap();
        let mut zeros: u64 = u64::from_str_radix(&v.replace("X", "1"), 2).unwrap();
        zeros |= 2u64.pow(63) - 2u64.pow(35); // extend with ones
        Mask { zeros, ones }
    }
}

impl Mask {
    fn apply(&self, input: u64) -> u64 {
        (input | self.ones) & self.zeros
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT.as_bytes()), 165);
    }
}
