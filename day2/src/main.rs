fn main() {
    let input = include_str!("my_input.txt");
    println!("Part 1: {}", input.lines().map(part1).sum::<u32>());
    println!("Part 2: {}", input.lines().map(part2).sum::<u32>());
}

fn part1(round: &str) -> u32 {
    match round {
        "A X" => 3+1, "A Y" => 6+2, "A Z" => 0+3,
        "B X" => 0+1, "B Y" => 3+2, "B Z" => 6+3,
        "C X" => 6+1, "C Y" => 0+2, "C Z" => 3+3,
        _ => 0 // Should never happen
    }
}

fn part2(round: &str) -> u32 {
    match round {
        "A X" => 3+0, "A Y" => 1+3, "A Z" => 2+6,
        "B X" => 1+0, "B Y" => 2+3, "B Z" => 3+6,
        "C X" => 2+0, "C Y" => 3+3, "C Z" => 1+6,
        _ => 0 // Should never happen
    }
}