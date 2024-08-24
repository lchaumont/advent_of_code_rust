pub fn run(input: String) {
    let numbers: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();

    println!("Part 1: {:?}", part_1(&numbers));
    println!("Part 2: {:?}", part_2(&numbers));
}

fn part_1(numbers: &[u32]) -> Option<u32> {
    for (i, num1) in numbers.iter().enumerate() {
        for num2 in numbers[i..].iter() {
            if num1 + num2 == 2020 {
                return Some(num1 * num2);
            }
        }
    }

    None
}

fn part_2(numbers: &[u32]) -> Option<u32> {
    for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers[i..].iter().enumerate() {
            if num1 + num2 > 2020 {
                continue;
            }

            for num3 in numbers[j..].iter() {
                if num1 + num2 + num3 == 2020 {
                    return Some(num1 * num2 * num3);
                }
            }
        }
    }

    None
}