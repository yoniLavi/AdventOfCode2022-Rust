use std::collections::HashSet;

fn main() {
    let input = include_str!("my_input.txt");
    let sum = priority_sum(part1_iterator(input));
    println!("{}", sum);
}

fn part1_iterator(input: &str) -> impl Iterator<Item = char> + '_ {
    input.lines().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        let left_chars: HashSet<char> = left.chars().collect();
        let right_chars: HashSet<char> = right.chars().collect();
        *left_chars.intersection(&right_chars).next().unwrap()
    })
}

fn priority(common: char) -> u8 {
    match u8::try_from(common) {
        Ok(c) if c.is_ascii_lowercase() => c - b'a' + 1,
        Ok(c) => c - b'A' + 26 + 1,
        Err(_) => unreachable!("Unexpected char in input"),
    }
}

fn priority_sum(iterator: impl Iterator<Item = char>) -> u32 {
    iterator.map(priority).map(|n| n as u32).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
        assert_eq!(priority('P'), 42);
        assert_eq!(priority('v'), 22);
        assert_eq!(priority('t'), 20);
        assert_eq!(priority('s'), 19);
    }
}
