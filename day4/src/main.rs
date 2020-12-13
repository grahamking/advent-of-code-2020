use std::collections::HashMap;
use std::fs;

use regex::Regex;

struct Cred<'a, F: Fn(&str) -> bool> {
    m: HashMap<&'a str, &'a str>,
    r: &'a HashMap<&'static str, F>,
}

impl<'a, F: Fn(&str) -> bool> Cred<'a, F> {
    fn add_fields(&mut self, l: &'a str) {
        for entry in l.split_whitespace() {
            let parts = entry.split(":").collect::<Vec<&'a str>>();
            self.m.insert(parts[0], parts[1]);
        }
    }
    fn is_valid(&self) -> bool {
        for (key, f) in self.r {
            let val = match self.m.get(key) {
                Some(v) => v,
                None => {
                    return false;
                }
            };
            if !f(val) {
                return false;
            }
        }
        true
    }
}

fn in_years(after: usize, before: usize) -> Box<dyn Fn(&str) -> bool> {
    return Box::new(move |x: &str| -> bool {
        let y = x.parse::<usize>().unwrap_or(0);
        after <= y && y <= before
    });
}

fn matches(re: &str) -> Box<dyn Fn(&str) -> bool> {
    let full_re = String::from("^") + re + "$";
    let r = Regex::new(&full_re).unwrap();
    return Box::new(move |x: &str| -> bool { r.is_match(x) });
}

fn height() -> Box<dyn Fn(&str) -> bool> {
    let r = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
    return Box::new(move |x: &str| -> bool {
        let cap = match r.captures(x) {
            Some(c) => c,
            None => return false,
        };
        let val = cap.get(1).unwrap().as_str().parse::<usize>().unwrap_or(0);
        let unit = cap.get(2).unwrap().as_str();
        if unit == "cm" {
            return 150 <= val && val <= 193;
        } else {
            //regexp ensures it's one or the other
            return 59 <= val && val <= 76;
        }
    });
}

fn main() {
    let mut req = HashMap::new();
    req.insert("byr", in_years(1920, 2002));
    req.insert("iyr", in_years(2010, 2020));
    req.insert("eyr", in_years(2020, 2030));
    req.insert("hgt", height());
    req.insert("hcl", matches(r"#[0-9a-f]{6}"));
    req.insert("ecl", matches(r"amb|blu|brn|gry|grn|hzl|oth"));
    req.insert("pid", matches(r"\d{9}"));

    let mut num_valid = 0;
    let mut cred = Cred {
        m: HashMap::new(),
        r: &req,
    };
    let input = fs::read_to_string("input.txt").unwrap();
    for l in input.lines() {
        if l.trim().len() == 0 {
            if cred.is_valid() {
                num_valid += 1;
            }
            //println!("{:?}", cred.m);
            cred = Cred {
                m: HashMap::new(),
                r: &req,
            };
        }
        cred.add_fields(l);
    }
    if cred.is_valid() {
        num_valid += 1;
    }
    println!("{}", num_valid);
}
