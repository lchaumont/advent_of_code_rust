use std::collections::HashSet;

pub fn run(input: String) {
    let groups: Vec<Vec<&str>> = input
        .split("\n\r\n")
        .map(|x| x.split_whitespace().collect())
        .collect();

    println!("Part 1: {:?}", part_1(&groups));
    println!("Part 2: {:?}", part_2(&groups));
}

fn part_1(groups: &Vec<Vec<&str>>) -> usize {
    let mut count = 0;

    for group in groups {
        let mut yes: HashSet<char> = HashSet::new();

        group.iter().for_each(|person_answers| {
            person_answers.chars().for_each(|answer| {
                yes.insert(answer);
            })
        });

        count += yes.len();
    }

    count
}

fn part_2(groups: &Vec<Vec<&str>>) -> usize {
    let mut count = 0;

    for group in groups {
        let mut yes: HashSet<char> = HashSet::new();

        group[..1].get(0).unwrap().chars().for_each(|answer| {
            yes.insert(answer);
        });

        group[1..].iter().for_each(|person_answers| {
            yes.retain(|x| person_answers.contains(*x));
        });

        count += yes.len();
    }

    count
}