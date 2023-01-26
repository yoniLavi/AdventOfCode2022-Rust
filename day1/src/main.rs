use fixed_heap::FixedHeap;

fn main() {
    let input = include_str!("my_input.txt");
    println!("Part 1 (top elf): {}", top_n_elves::<1>(input));
    println!("Part 2 (sum of 3 top elves): {}", top_n_elves::<3>(input));
}

fn top_n_elves<const N: usize>(input: &str) -> u32 {
    let mut elf_sum = 0;
    let mut top_elves: FixedHeap<u32, N> = FixedHeap::default();
    let comparer = |a: &u32, b: &u32, _: &()| a > b;

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => {
                elf_sum += n;
            },
            Err(_) => { // Assume blank line and finish processing this elf
                top_elves.push(elf_sum, &comparer, &());
                elf_sum = 0;
            }
        }
    }
    top_elves.iter().sum()
}
