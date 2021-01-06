fn main() {
    println!("Part 1: {}", part1(15_733_400, 6_408_062));
}

fn part1(card: usize, door: usize) -> usize {
    (0..loop_size(door)).fold(1, |acc, _| (acc * card) % 20201227)
}

fn loop_size(target: usize) -> usize {
    let mut x = 1;
    let mut loop_count = 0;
    while x != target {
        x = (x * 7) % 20201227;
        loop_count += 1;
    }
    loop_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_loop_size() {
        assert_eq!(super::loop_size(5764801), 8);
        assert_eq!(super::loop_size(17807724), 11);
    }
    #[test]
    fn test_part1() {
        assert_eq!(super::part1(5764801, 17807724), 14897079);
    }
}
