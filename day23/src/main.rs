use std::char::from_digit;
use std::collections::VecDeque;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", part1("562893147")?);
    Ok(())
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut v: VecDeque<u32> = input.chars().filter_map(|x| x.to_digit(10)).collect();

    (1..=100).for_each(|_| a_move(&mut v).unwrap());

    let pos_one = v.iter().position(|&x| x == 1).unwrap();
    v.rotate_left(pos_one);
    let s: String = v
        .iter()
        .skip(1)
        .map(|x| from_digit(*x, 10).unwrap())
        .collect();

    Ok(s)
}

fn a_move(v: &mut VecDeque<u32>) -> Result<(), Box<dyn Error>> {
    v.rotate_left(1);
    let current = *v.back().ok_or("v is empty")?;

    let mut pick_three = Vec::new();
    (1..=3).for_each(|_| pick_three.push(v.pop_front().unwrap()));

    let dest_idx = destination(current, &v)?;

    for (i, p) in pick_three.iter().enumerate() {
        v.insert(dest_idx + i, *p);
    }
    Ok(())
}

fn destination(current: u32, v: &VecDeque<u32>) -> Result<usize, Box<dyn Error>> {
    let min = v.iter().min().ok_or("no min")?;
    let mut target = current - 1;
    let dest_idx = 'outer: loop {
        for (idx, val) in v.iter().enumerate() {
            if *val == target {
                break 'outer idx;
            }
        }
        if target < *min {
            let max = v.iter().max().ok_or("no max")?;
            break v.iter().position(|&x| x == *max).unwrap();
        }
        target -= 1;
    };
    Ok(dest_idx + 1)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(crate::part1("389125467")?, "67384529");
        Ok(())
    }
}
