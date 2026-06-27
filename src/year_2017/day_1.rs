pub fn run(input: &str) {
    let numbers: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!("Part 1: {}", part_1(&numbers));
    println!("Part 2: {}", part_2(&numbers));
}

fn part_1(numbers: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        if numbers[i] == numbers[(i + 1) % numbers.len()] {
            sum += numbers[i];
        }
    }
    sum
}

fn part_2(numbers: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        if numbers[i] == numbers[(i + numbers.len() / 2) % numbers.len()] {
            sum += numbers[i];
        }
    }
    sum
}
