pub fn run(input: String) {
    let chars: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    println!("Part 1: {:?}", part_1(&chars));
    println!("Part 2: {:?}", part_2(&chars));
}

fn get_char_at(chars: &Vec<Vec<char>>, x: usize, y: usize, width: usize, height: usize) -> &char {
    let x_target = x % width;
    let y_target = y % height;

    let row = chars.get(y_target).unwrap();
    row.get(x_target).unwrap()
}

fn part_1(chars: &Vec<Vec<char>>) -> u32 {
    let width_of_pattern = chars.first().unwrap().len();
    let height_of_pattern = chars.len();

    let mut nbr_of_tree_encountered = 0;

    for h in 0..height_of_pattern {
        let char = get_char_at(chars, h * 3, h, width_of_pattern, height_of_pattern);
        if *char == '#' {
            nbr_of_tree_encountered += 1;
        }
    }

    nbr_of_tree_encountered
}

fn part_2(chars: &Vec<Vec<char>>) -> u64 {
    let width_of_pattern = chars.first().unwrap().len();
    let height_of_pattern = chars.len();

    let mut results: Vec<u64> = Vec::new();
    let combinations: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (step_x, step_y) in combinations {
        let mut nbr_of_tree_encountered = 0;

        for h in (0..height_of_pattern).step_by(step_y) {
            let char = get_char_at(chars, (h * step_x) / step_y, h, width_of_pattern, height_of_pattern);
            if *char == '#' {
                nbr_of_tree_encountered += 1;
            }
        }

        results.push(nbr_of_tree_encountered);
    }

    results.iter().copied().reduce(|acc, e| acc * e).unwrap()
}