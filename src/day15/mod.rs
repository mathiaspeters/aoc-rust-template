pub fn day15() {
    println!("Result 15-1: {}", part1());
    println!("Result 15-2: {}", part2());
}

fn part1() -> usize {
    0
}

fn part2() -> usize {
    0
}

#[cfg(not(test))]
fn raw_input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input() -> &'static str {
    include_str!("testinput")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(0, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2());
    }
}