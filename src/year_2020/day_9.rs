use std::collections::VecDeque;

pub fn run(input: String) {
    let numbers: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();

    let part_1_result = part_1(&numbers);
    println!("Part 1: {:?}", part_1(&numbers));

    match part_1_result {
        None => {}
        Some(target) => {
            println!("Part 2: {:?}", part_2(&numbers, target));
        }
    }
}

fn part_1(numbers: &[usize]) -> Option<usize> {
    let queue_size: usize = 25;
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(queue_size);

    // Init the queue with `queue_size` elements;
    for i in 0..numbers.len() {
        let current = numbers[i];

        if (i < queue_size) {
            queue.push_back(current)
        } else {
            let mut found = false;

            'OUTER: for j in 0..queue.len() {
                let at_j = queue[j];
                for k in j..queue.len() {
                    let at_k = queue[k];

                    if at_j + at_k == current {
                        found = true;
                        break 'OUTER;
                    }
                }
            }

            if found {
                queue.pop_front();
                queue.push_back(current);
            } else {
                return Some(current);
            }
        }
    }

    None
}

fn part_2(numbers: &[usize], target: usize) -> Option<usize> {
    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut sum = 0;

    for number in numbers {
        queue.push_back(*number);
        sum += number;

        while sum > target {
            match queue.pop_front() {
                None => {
                    break;
                }
                Some(value) => {
                    sum -= value;
                }
            }
        }

        if sum == target {
            let min = queue.iter().min().unwrap();
            let max = queue.iter().max().unwrap();

            return Some(min + max)
        }
    }

    None
}