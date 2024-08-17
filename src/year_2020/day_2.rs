pub fn run(input: String) {
    let data: Vec<Vec<String>> = input.lines().map(|x| {
        x.split(['-', ' ', ':'])
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string()).collect()
    }).collect();

    println!("Part 1: {:?}", part_1(&data));
    println!("Part 2: {:?}", part_2(&data));
}

fn part_1(data: &Vec<Vec<String>>) -> u32 {
    let mut result: u32 = 0;

    for line in data {
        let min = line[0].parse::<u32>().unwrap();
        let max = line[1].parse::<u32>().unwrap();
        let letter = line[2].chars().next().unwrap();
        let password = &line[3];

        let count = password.chars().filter(|&c| c == letter).count() as u32;

        if count >= min && count <= max {
            result += 1;
        }
    }

    result
}

fn part_2(data: &Vec<Vec<String>>) -> u32 {
    let mut result: u32 = 0;

    for line in data {
        let first_position = line[0].parse::<usize>().unwrap() - 1;
        let second_position = line[1].parse::<usize>().unwrap() - 1;
        let letter = line[2].chars().next().unwrap();
        let password = &line[3];

        let first = password.chars().nth(first_position);
        let second = password.chars().nth(second_position);

        if (first == Some(letter) && second != Some(letter)) || (first != Some(letter) && second == Some(letter)) {
            result += 1;
        }
    }

    result
}