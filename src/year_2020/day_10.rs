use std::collections::HashMap;

pub fn run(input: String) {
    let mut numbers: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    numbers.push(0);
    numbers.sort();

    println!("Part 1: {:?}", part_1(&numbers));
    println!("Part 2: {:?}", part_2(&numbers));
}

fn part_1(numbers: &[usize]) -> usize {
    let mut number_of_one_jolt_diff: usize = 0;
    let mut number_of_three_jolt_diff: usize = 1;

    for index in 0..numbers.len() {
        let current = numbers.get(index).unwrap();
        match numbers.get(index + 1) {
            None => {}
            Some(next) => {
                match next - current {
                    1 => {
                        number_of_one_jolt_diff += 1;
                    }
                    3 => {
                        number_of_three_jolt_diff += 1;
                    }
                    _ => {
                        eprintln!("Not found next adapters : {}", next - current)
                    }
                }
            }
        }
    }

    number_of_one_jolt_diff * number_of_three_jolt_diff
}

fn part_2(numbers: &[usize]) -> usize {
    let mut memo: HashMap<usize, usize> = HashMap::new();
    get_nbr_of_sub_path(numbers, &mut memo, 0)
}


fn get_nbr_of_sub_path(numbers: &[usize], memo: &mut HashMap<usize, usize>, current_index: usize) -> usize {
    match memo.get(&current_index) {
        None => {}
        Some(memoized) => {
            return *memoized
        }
    }

    let mut count: usize = 0;

    let current = numbers.get(current_index).unwrap();
    let next_possible_indexes: Vec<usize> = (current_index + 1..=current_index + 3).filter(|x| {
        match numbers.get(*x) {
            None => false,
            Some(next) => {
                next - current <= 3
            }
        }
    }).collect();

    if next_possible_indexes.len() > 0 {
        for next_index in next_possible_indexes {
            count += get_nbr_of_sub_path(numbers, memo, next_index);
        }
    } else {
        count = 1;
    }

    memo.insert(current_index, count);
    count
}