use std::collections::HashSet;

pub fn run(input: String) {
    let data: Vec<OPERATION> = input.lines().map(|x| {
        let mut split = x.split(" ");
        let operation = split.next().unwrap();
        let value = split.next().unwrap().parse::<isize>().unwrap();

        match operation {
            "nop" => OPERATION::NOP(value),
            "acc" => OPERATION::ACC(value),
            "jmp" => OPERATION::JMP(value),
            _ => panic!("Invalid operation")
        }
    }).collect();

    println!("Part 1: {:?}", part_1(&data));
    println!("Part 2: {:?}", part_2(&data));
}

#[derive(Clone, Copy, Debug)]
enum OPERATION {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

fn part_1(data: &Vec<OPERATION>) -> isize {
    solve(data).0
}

fn solve(data: &Vec<OPERATION>) -> (isize, usize) {
    let mut index_already_visited: HashSet<usize> = HashSet::new();
    let mut accumulator: isize = 0;
    let mut next_index: usize = 0;

    while !index_already_visited.contains(&next_index) && next_index < data.len() {
        index_already_visited.insert(next_index);

        match data.get(next_index) {
            None => {
                eprintln!("Failed to get data at index {}", next_index)
            }
            Some(operation) => {
                match operation {
                    OPERATION::NOP(_) => {
                        next_index += 1;
                    }
                    OPERATION::ACC(value) => {
                        accumulator += value;
                        next_index += 1;
                    }
                    OPERATION::JMP(value) => {
                        next_index = (next_index as isize + value) as usize;
                    }
                }
            }
        }
    }

    (accumulator, next_index)
}

fn part_2(data: &Vec<OPERATION>) -> isize {
    let mut lines: Vec<OPERATION> = data.clone();

    for index in 0..lines.len() {
        match lines[index] {
            OPERATION::JMP(v) => {
                lines[index] = OPERATION::NOP(v);
                let result: (isize, usize) = solve(&lines);
                if result.1 == lines.len() {
                    return result.0;
                }
                lines[index] = OPERATION::JMP(v);
            }
            OPERATION::NOP(v) => {
                lines[index] = OPERATION::JMP(v);
                let result: (isize, usize) = solve(&lines);
                if result.1 == lines.len() {
                    return result.0;
                }
                lines[index] = OPERATION::NOP(v);
            }
            _ => {}
        }
    }

    0
}