use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("input.txt")?;
    let mut seat_ids: Vec<usize> = BufReader::new(f)
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse(&x).2)
        .collect();
    seat_ids.sort();

    let mut iter = seat_ids.iter();
    let mut prev = iter.next().unwrap();
    for x in iter {
        if *x != prev + 1 {
            println!("{}", x - 1);
            break;
        }
        prev = x;
    }

    Ok(())
}

// Returns row, col, product.
fn parse(x: &str) -> (usize, usize, usize) {
    let row = chop(x, 0f32, 127f32, 0..7);
    let col = chop(x, 0f32, 7f32, 7..10);
    (row, col, row * 8 + col)
}

fn chop(x: &str, min_in: f32, max_in: f32, range: Range<usize>) -> usize {
    let mut min = min_in;
    let mut max = max_in;
    let mid = (min + max) / 2f32;
    let mut mid = mid.floor();
    for c in x.get(range).unwrap_or("").chars() {
        match c {
            'F' | 'L' => max = mid,
            'B' | 'R' => min = mid,
            _ => panic!("invalid c {}", c),
        }
        mid = (min + max) / 2f32;
        mid = mid.floor();
    }
    max as usize
}
