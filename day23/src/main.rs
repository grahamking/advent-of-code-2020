use std::collections::VecDeque;
use std::error::Error;

static DEBUG: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", part1("562893147")?);
    Ok(())
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut v: VecDeque<u32> = input.chars().filter_map(|x| x.to_digit(10)).collect();

    for i in 1..=100 {
        let current = v.pop_front().ok_or("v is empty")?;
        v.push_back(current);

        let rest = v.split_off(3);
        let pick_three = v;
        v = rest;

        // choose destination
        let min = *v.iter().min().ok_or("no min")?;
        let mut target = current - 1;
        let mut dest_idx = 'outer: loop {
            for (idx, val) in v.iter().enumerate() {
                if *val == target {
                    break 'outer idx;
                }
            }
            if target < min {
                let max = *v.iter().max().ok_or("no max")?;
                break v.iter().position(|&x| x == max).unwrap();
            }
            target -= 1;
        };
        dest_idx += 1;

        for (i, p) in pick_three.iter().enumerate() {
            v.insert(dest_idx + i, *p);
        }

        if DEBUG {
            println!("-- move {}, current {} --", i, current);
            println!("cups: {:?}", v);
            println!("pick up: {:?}", pick_three);
            println!("dest_idx: {}", dest_idx);
            println!();
        }
    }

    let pos_one = v.iter().position(|&x| x == 1).unwrap();
    v.rotate_left(pos_one);
    v.pop_front(); // remove digit 1
    let s: String = v
        .iter()
        .map(|x| std::char::from_digit(*x, 10).unwrap())
        .collect();

    Ok(s)
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
