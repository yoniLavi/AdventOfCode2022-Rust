fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = include_str!("my_input.txt");

    let mut elf_sum = 0;
    let mut max_sum = 0;
    for line in contents.lines() {
        match line.parse::<i32>() {
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
    println!("{}", max_sum);
}

fn part2() {
    let contents = include_str!("my_input.txt");

    let mut elf_sum = 0;
    let mut max_three = vec![0, 0, 0];
    for line in contents.lines() {
        match line.parse::<i32>() {
            Ok(n) => {
                elf_sum += n;
            },
            Err(_) => { // Assume blank line and finish processing this elf
                max_three.push(elf_sum);
                max_three.sort();
                max_three = (&max_three[1..4]).to_vec();
                elf_sum = 0;
            }
        }
    }
    println!("{}", max_three.iter().sum::<i32>());
}
