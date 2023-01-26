use fixed_heap::FixedHeap;

fn main() {
    let input = include_str!("my_input.txt");

    println!("Part 1 (top elf): {}", part1(input));
    println!("Part 2 (sum of 3 top elves): {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut elf_sum = 0;
    let mut max_sum = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => {
                elf_sum += n;
            },
            Err(_) => { // The only error we expect is a blank line
                if elf_sum > max_sum {
                    max_sum = elf_sum;
                }
                elf_sum = 0;
            }
        }
    }
    max_sum
}

fn part2(input: &str) -> u32 {
    let mut elf_sum = 0;
    let mut top_3_elves: FixedHeap<u32, 3> = FixedHeap::default();
    let heap_comparer = |a: &u32, b: &u32, _: &()| a > b;

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => {
                elf_sum += n;
            },
            Err(_) => { // Assume blank line and finish processing this elf
                top_3_elves.push(elf_sum, &heap_comparer, &());
                elf_sum = 0;
            }
        }
    }
    top_3_elves.iter().sum()
}
