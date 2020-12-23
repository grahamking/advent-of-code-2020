use std::cmp::Ordering;
use std::fs;

fn main() {
    let mut input: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    input.sort_unstable();
    let l = input.len();

    for (i, val1) in input.iter().enumerate() {
        println!("{} {}", i, val1);
        'outer: for j in i + 1..l {
            let val2 = input.get(j).unwrap();
            for k in j + 1..l {
                let val3 = input.get(k).unwrap();
                let sum = val1 + val2 + val3;
                match sum.cmp(&2020) {
                    Ordering::Less => continue,
                    Ordering::Greater => continue 'outer,
                    _ => {}
                }
                println!(
                    "ANSWER: {} {} {} = {}",
                    val1,
                    val2,
                    val3,
                    val1 * val2 * val3
                );
                return;
            }
        }
    }
}
