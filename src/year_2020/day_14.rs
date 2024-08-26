use std::collections::HashMap;

pub fn run(input: String) {
    let instructions: Vec<Instruction> = input.lines().map(|x| {
        let mut parts = x.split(" = ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        if left.starts_with("mask") {
            Instruction::MASK(right.to_string())
        } else {
            let mem = left.split("[").nth(1).unwrap().split("]").nth(0).unwrap().parse().unwrap();
            let value = right.parse().unwrap();
            Instruction::Mem(mem, value)
        }
    }).collect();

    println!("Part 1: {:?}", part_1(&instructions));
    println!("Part 2: {:?}", part_2(&instructions));
}

#[derive(Debug)]
enum Instruction {
    MASK(String),
    Mem(usize, usize),
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut result: HashMap<usize, usize> = HashMap::new();

    let mut current_mask = "";
    for instruction in instructions {
        match instruction {
            Instruction::MASK(mask) => {
                current_mask = mask;
            }
            Instruction::Mem(mem, value) => {
                let mut binary = format!("{:036b}", value);

                for (i, c) in current_mask.chars().enumerate() {
                    if c != 'X' {
                        binary = binary.chars().enumerate().map(|(j, x)| {
                            if j == i {
                                c
                            } else {
                                x
                            }
                        }).collect();
                    }
                }

                result.insert(*mem, usize::from_str_radix(&binary, 2).unwrap());
            }
        }
    }

    result.values().sum()
}

fn part_2(instructions: &Vec<Instruction>) -> usize {
    let mut result: HashMap<usize, usize> = HashMap::new();

    let mut current_mask = "";

    for instruction in instructions {
        match instruction {
            Instruction::MASK(mask) => {
                current_mask = mask;
            }

            Instruction::Mem(mem, value) => {
                let mut binary = format!("{:036b}", mem);
                let mut floating: Vec<usize> = vec![];

                for (i, c) in current_mask.chars().enumerate() {
                    binary = binary.chars().enumerate().map(|(j, x)| {
                        if j == i {
                            if c == 'X' {
                                floating.push(i);
                                '0'
                            } else if c == '1' {
                                '1'
                            } else {
                                x
                            }
                        } else {
                            x
                        }
                    }).collect();
                }

                for i in 0..2_usize.pow(floating.len() as u32) {
                    let mut binary = binary.clone();
                    let floating = floating.clone();

                    for (j, c) in format!("{:0b}", i).chars().rev().enumerate() {
                        if c == '1' {
                            binary = binary.chars().enumerate().map(|(k, x)| {
                                if k == floating[j] {
                                    '1'
                                } else {
                                    x
                                }
                            }).collect();
                        }
                    }

                    result.insert(usize::from_str_radix(&binary, 2).unwrap(), *value);
                }
            }
        }
    }

    result.values().sum()
}