fn main() {
    let input = include_str!("my_input.txt");
    println!("Part 1 (top elf): {}", process_elves(input, 1));
    println!("Part 2 (sum of 3 top elves): {}", process_elves(input, 3));
}

fn process_elves(input: &str, top_n: usize) ->u32 {
    let mut elf_sum = 0; // Sum of calories for current elf
    let mut top_elves = vec![0; top_n]; // Calories for the top `top_n` elves

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => {
                elf_sum += n;
            },
            Err(_) => { // Assume blank line and finish processing this elf
                top_elves.push(elf_sum);
                top_elves.sort();
                top_elves = (top_elves[1..(top_n+1)]).to_vec();
                elf_sum = 0;
            }
        }
    }
    return top_elves.iter().sum();
}
