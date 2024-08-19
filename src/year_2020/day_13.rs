use std::process::id;

pub fn run(input: String) {
    let mut splitted = input.split("\r\n");
    let timestamp = splitted.next().unwrap().parse::<usize>().unwrap();
    let bus_ids: Vec<&str> = splitted.next().unwrap().split(",").collect();

    println!("Part 1: {:?}", part_1(timestamp, &bus_ids));
    println!("Part 2: {:?}", part_2(&bus_ids));
}

fn part_1(timestamp: usize, bus_ids: &Vec<&str>) -> usize {
    let answer = bus_ids.iter()
        .filter(|id| **id != "x")
        .map(|id| {
            let id_nbr = id.parse::<usize>().unwrap();
            (id_nbr, id_nbr - (timestamp % id_nbr))
        })
        .min_by(|id1, id2| id1.1.cmp(&id2.1))
        .unwrap();

    answer.0 * answer.1
}

fn part_2(bus_ids: &Vec<&str>) -> usize {
    let ids: Vec<usize> = bus_ids.iter()
        .map(|id| id.parse::<usize>().unwrap_or(0))
        .collect();

    let mut timestamp: usize = 0;
    let mut step: usize = 1;

    for (offset, id) in ids.iter().enumerate() {
        if *id == 0 { continue; }

        for index in (timestamp..usize::MAX).step_by(step) {
            if (index + offset) % id == 0 {
                timestamp = index;
                step *= id;
                break;
            }
        }
    }

    timestamp
}