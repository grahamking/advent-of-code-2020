use std::boxed::Box;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    println!("Part 1: {}", part1(&input)?);
    let second_line = input.lines().skip(1).next().ok_or("missing second")?;
    println!("Part 2: {}", part2(&second_line)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut l = input.lines();
    let ts: usize = l.next().ok_or("missing first line")?.parse()?;
    let schedule: Vec<usize> = l
        .next()
        .ok_or("missing second line")?
        .split(",")
        .filter_map(|x| x.parse().ok()) // skip the 'x'
        .collect();

    let mut min_min = usize::MAX;
    let mut min_id = 0;
    for val in schedule {
        let m = val - (ts % val);
        if m < min_min {
            min_min = m;
            min_id = val;
        }
    }
    Ok(min_min * min_id)
}

fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    // from https://gist.github.com/miseran/abf1629c6498538a0175ff7548635317
    let busses: Vec<(i64, i64)> = input
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|b| (i as i64, b)))
        .collect();
    let prod: i64 = busses.iter().map(|(_, b)| b).product();

    let result2 = busses
        .iter()
        .map(|&(a, b)| -a * (prod / b) * inv_mod(prod / b, b))
        .sum::<i64>()
        .rem_euclid(prod);

    Ok(result2)
}

fn inv_mod(x: i64, p: i64) -> i64 {
    // p must be prime
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&"939\n7,13,x,x,59,x,31,19").unwrap(), 295);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&"17,x,13,19").unwrap(), 3417);
        assert_eq!(super::part2(&"7,13,x,x,59,x,31,19").unwrap(), 1068781);
        assert_eq!(super::part2(&"67,x,7,59,61").unwrap(), 779210);
        assert_eq!(super::part2(&"67,7,x,59,61").unwrap(), 1261476);
        assert_eq!(super::part2(&"1789,37,47,1889").unwrap(), 1202161486);
    }
}
