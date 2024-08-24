use std::collections::HashMap;

pub fn run(input: String) {
    let numbers: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();

    println!("Part 1: {:?}", solve(&numbers, 2020));
    println!("Part 2: {:?}", solve(&numbers, 30_000_000));
}

fn solve(numbers: &Vec<i32>, end_index: usize) -> i32 {
    let mut vec_index: Vec<i32> = vec![-1; end_index];
    let mut last_spoken: i32 = *numbers.last().unwrap();

    numbers.iter().enumerate().for_each(|(index, &n)| {
        vec_index[n as usize] = index as i32;
    });

    for index in numbers.len()..end_index {
        let next = match vec_index[last_spoken as usize] {
            -1 => 0,
            last_index => index as i32 - 1 - last_index
        };

        vec_index[last_spoken as usize] = index as i32 - 1;
        last_spoken = next;
    }

    last_spoken
}

/*
fn solve(numbers: &Vec<i32>, end_index: usize) -> i32 {
    let mut map_number_turn_index: HashMap<i32, i32> = HashMap::new();
    let mut last_spoken: (i32, Option<i32>) = (*numbers.last().unwrap(), None);

    numbers.iter().enumerate().for_each(|(index, n)| { map_number_turn_index.insert(*n, index as i32 + 1); });

    for index in (numbers.len() + 1)..=end_index {
        let next = match last_spoken.1 {
            None => 0, // First time it was said, so 0
            Some(last_index) => index as i32 - 1 - last_index,
        };
        last_spoken = (next, map_number_turn_index.get(&next).copied());
        map_number_turn_index.insert(next, index as i32);
    }

    last_spoken.0
}*/