use std::collections::HashSet;
use std::fs::read_to_string;

// This one feels hacky, there's surely a far more elegant way to do this.
// However it takes 0m0.007s to run with the full input on my laptop.
// Hard to justify optimizing!

fn main() {
    let mut groups = Vec::new();
    let mut all = HashSet::new();
    let mut first_of_group = true;
    for line in read_to_string("input.txt").unwrap().lines() {
        if line.len() == 0 {
            groups.push(all);
            all = HashSet::new();
            first_of_group = true;
            continue;
        }
        let mut here = HashSet::new();
        for c in line.chars() {
            here.insert(c);
        }
        if first_of_group {
            all = here.clone();
            first_of_group = false;
        } else {
            all = all.intersection(&here).cloned().collect();
            // if the first two lines of a group have no overlap,
            // we could skip to the next group
        }
    }
    groups.push(all);
    println!(
        "{}",
        groups.iter().map(|x| x.len()).fold(0, |acc, x| acc + x)
    );
}
