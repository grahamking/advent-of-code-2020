use std::fmt;
use std::fs;

struct Policy<'a> {
    raw: &'a str,
    first: usize,
    second: usize,
    c: char,
    pw: &'a str,
}

impl<'a> Policy<'a> {
    fn ok(&self) -> bool {
        let cs: Vec<char> = self.pw.chars().collect();
        let y1 = cs[self.first - 1] == self.c && cs[self.second - 1] != self.c;
        let y2 = cs[self.first - 1] != self.c && cs[self.second - 1] == self.c;
        y1 || y2

        //let n = self.pw.chars().filter(|c| *c == self.c).count();
        //self.first <= n && n <= self.second
    }
}

impl<'a> From<&'a str> for Policy<'a> {
    fn from(s: &'a str) -> Self {
        // s is e.g. "1-3 a: abcde"
        let parts = s.split(' ').collect::<Vec<&str>>();
        let mut min_max = parts[0].split('-').map(|x| x.parse().unwrap());
        Policy {
            raw: s,
            first: min_max.next().unwrap(),
            second: min_max.next().unwrap(),
            c: parts[1].chars().next().unwrap(),
            pw: parts[2].trim(),
        }
    }
}

impl<'a> fmt::Debug for Policy<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n  {} {} {} {} = {}",
            self.raw,
            self.first,
            self.second,
            self.c,
            self.pw,
            self.ok(),
        )
    }
}

fn main() {
    let mut num = 0;
    for l in fs::read_to_string("input.txt").unwrap().lines() {
        let policy: Policy = l.into();
        if policy.ok() {
            num += 1;
        }
    }
    println!("{}", num);
}
